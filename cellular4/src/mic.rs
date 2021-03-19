use std::{
    mem,
    sync::{Arc, Mutex},
};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Sample, SampleFormat, Stream, StreamConfig,
};
use failure::{format_err, Fallible};
use itertools::{izip, Itertools};
use lerp::Lerp;
use odds::stride::Stride;
use realfft::{num_complex::Complex, RealFftPlanner, RealToComplex};

use crate::prelude::*;

pub struct FrequencyHistogram {
    current: Vec<f32>,
    next: Vec<f32>,
    max: f32,
}

impl FrequencyHistogram {
    pub fn new(n_bins: usize) -> Self {
        Self {
            current: vec![0.0; n_bins],
            next: vec![0.0; n_bins],
            max: 0.0,
        }
    }

    pub fn bins(&self) -> &[f32] {
        &self.current
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn get_normalised(&self, index: usize) -> UNFloat {
        let v = self.current[index] / self.max;
        if v.is_normal() {
            UNFloat::new(v)
        } else {
            UNFloat::ZERO
        }
    }
}

pub struct FrequencyHistograms {
    linear: FrequencyHistogram,
    gamma: FrequencyHistogram,
}

impl FrequencyHistograms {
    pub fn new(n_bins: usize) -> Self {
        Self {
            linear: FrequencyHistogram::new(n_bins),
            gamma: FrequencyHistogram::new(n_bins),
        }
    }

    pub fn get_histogram(&self, gamma: bool) -> &FrequencyHistogram {
        if gamma {
            &self.gamma
        } else {
            &self.linear
        }
    }
}

pub struct FftMicReader {
    config: MicConfig,

    mic: MicReader,

    fft: Arc<dyn RealToComplex<f32>>,

    fft_in_buf: Vec<f32>,
    fft_out_buf: Vec<Complex<f32>>,
    fft_scratch: Vec<Complex<f32>>,
    window: Vec<f32>,

    adj_lerp_factor: f32,
    min_frequency_idx: usize,
    max_frequency_idx: usize,
    norm: f32,
}

impl FftMicReader {
    pub fn new(config: MicConfig) -> Fallible<Self> {
        let mic = MicReader::build(|stream_config| {
            usize::from(stream_config.channels)
                * chunk_size_for_fps(stream_config.sample_rate.0, config.target_fps)
        })?;

        let stream_config = mic.config();
        let sample_rate = stream_config.sample_rate.0;
        let channel_chunk_size = mic.chunk_size() / usize::from(stream_config.channels);

        let mut planner = RealFftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(channel_chunk_size);

        let fft_in_buf = fft.make_input_vec();
        let fft_out_buf = fft.make_output_vec();
        let fft_scratch = fft.make_scratch_vec();

        let window = apodize::hamming_iter(fft_in_buf.len())
            .map(|w| w as f32)
            .collect::<Vec<_>>();

        let adj_lerp_factor = config
            .lerp_factor
            .powf(fft_in_buf.len() as f32 / sample_rate as f32);

        let norm = 1.0 / (fft_in_buf.len() as f32).sqrt();

        let min_frequency_idx =
            frequency_to_fft_idx(config.min_frequency, sample_rate, fft_out_buf.len());
        let max_frequency_idx =
            frequency_to_fft_idx(config.max_frequency, sample_rate, fft_out_buf.len());

        Ok(Self {
            config,
            mic,
            fft,
            fft_in_buf,
            fft_out_buf,
            fft_scratch,
            window,
            adj_lerp_factor,
            min_frequency_idx,
            max_frequency_idx,
            norm,
        })
    }

    pub fn update(&mut self, histograms: &mut FrequencyHistograms) -> Fallible<()> {
        let stream_config = self.mic.config();
        let num_channels = stream_config.channels as usize;

        let mut first = true;

        while let Some(chunk) = self.mic.next_chunk() {
            if first {
                for histogram in &mut [&mut histograms.linear, &mut histograms.gamma] {
                    for next_bin in histogram.next.iter_mut() {
                        *next_bin = 0.0;
                    }
                }

                first = false;
            }

            assert_eq!(self.fft_in_buf.len(), chunk.len() / num_channels);

            for channel_i in 0..num_channels {
                for (fft_sample, w, sample) in izip!(
                    self.fft_in_buf.iter_mut(),
                    self.window.iter(),
                    Stride::from_slice(&chunk[channel_i..], num_channels as isize),
                ) {
                    *fft_sample = *w * sample;
                }

                self.fft
                    .process_with_scratch(
                        &mut self.fft_in_buf,
                        &mut self.fft_out_buf,
                        &mut self.fft_scratch,
                    )
                    .map_err(|e| format_err!("Failed computing FFT: {}", e))?;

                let select_fft = &self.fft_out_buf[self.min_frequency_idx..self.max_frequency_idx];

                let mut prev_lin_bin_idx = 0;
                let mut prev_gamma_bin_idx = 0;

                for (fft_idx, fft_sample) in select_fft.iter().enumerate() {
                    let scaled = fft_sample * self.norm;
                    let power = scaled.norm_sqr();
                    let power_log = power.ln();

                    for (gamma, histogram, prev_bin_idx) in &mut [
                        (
                            self.config.gamma,
                            &mut histograms.gamma,
                            &mut prev_gamma_bin_idx,
                        ),
                        (1.0, &mut histograms.linear, &mut prev_lin_bin_idx),
                    ] {
                        let bin_idx = gamma_corrected_bin_idx(
                            fft_idx,
                            select_fft.len(),
                            self.config.min_frequency,
                            self.config.max_frequency,
                            histogram.next.len(),
                            *gamma,
                        );

                        let bin_range = (**prev_bin_idx + 1).min(bin_idx)..=bin_idx;

                        for next_bin in &mut histogram.next[bin_range] {
                            *next_bin = f32::max(*next_bin, power_log);
                        }

                        **prev_bin_idx = bin_idx;
                    }
                }

                for histogram in &mut [&mut histograms.linear, &mut histograms.gamma] {
                    for (bin, next_bin) in histogram.current.iter_mut().zip(histogram.next.iter()) {
                        *bin = bin.lerp(*next_bin, self.adj_lerp_factor);
                        histogram.max = histogram.max.max(*bin);
                    }
                }
            }
        }

        Ok(())
    }
}

fn chunk_size_for_fps(sample_rate: u32, fps: f32) -> usize {
    let approx = (sample_rate as f32 / fps).round() as usize;

    let mut n = 1;

    loop {
        let next = n * 2;
        if next > approx {
            return n;
        }
        n = next;
    }
}

fn frequency_to_fft_idx(freq: f32, sample_rate: u32, out_fft_buf_len: usize) -> usize {
    num::clamp(
        (2.0 * freq / sample_rate as f32 * (out_fft_buf_len - 1) as f32).round() as usize + 1,
        1,
        out_fft_buf_len,
    )
}

fn gamma_corrected_bin_idx(
    fft_idx: usize,
    fft_out_buf_len: usize,
    min_freq: f32,
    max_freq: f32,
    bins_len: usize,
    gamma: f32,
) -> usize {
    let r = fft_idx as f32 / fft_out_buf_len as f32;
    let s = min_freq.lerp(max_freq, r) / max_freq;
    let i = s.powf(1.0 / gamma) * bins_len as f32;
    num::clamp(i.round() as usize, 0, bins_len - 1)
}

pub enum MicReader {
    F32(StreamReader<f32>),
    I16(ConvertingStreamReader<i16, f32>),
    U16(ConvertingStreamReader<u16, f32>),
}

impl MicReader {
    pub fn build<F: FnMut(&StreamConfig) -> usize>(mut chunk_size_fn: F) -> Fallible<Self> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| format_err!("No input device available"))?;

        let config = device.default_input_config()?;
        let sample_format = config.sample_format();
        let config = config.config();

        let chunk_size = chunk_size_fn(&config);

        match sample_format {
            SampleFormat::F32 => Ok(MicReader::F32(StreamReader::new(
                device, config, chunk_size,
            )?)),
            SampleFormat::I16 => Ok(MicReader::I16(ConvertingStreamReader::new(
                device, config, chunk_size,
            )?)),
            SampleFormat::U16 => Ok(MicReader::U16(ConvertingStreamReader::new(
                device, config, chunk_size,
            )?)),
        }
    }

    pub fn chunk_size(&self) -> usize {
        match self {
            MicReader::F32(r) => r.chunk_size(),
            MicReader::I16(r) => r.chunk_size(),
            MicReader::U16(r) => r.chunk_size(),
        }
    }

    pub fn next_chunk(&mut self) -> Option<&mut [f32]> {
        match self {
            MicReader::F32(r) => r.next_chunk(),
            MicReader::I16(r) => r.next_chunk(),
            MicReader::U16(r) => r.next_chunk(),
        }
    }

    pub fn config(&self) -> &StreamConfig {
        match self {
            MicReader::F32(r) => r.config(),
            MicReader::I16(r) => r.config(),
            MicReader::U16(r) => r.config(),
        }
    }
}

pub struct StreamReader<T> {
    chunk_size: usize,
    back_buf: Arc<Mutex<Vec<T>>>,
    front_buf: Vec<T>,
    chunk_buf: Vec<T>,
    chunk_offset: usize,
    _stream: Stream,
    config: StreamConfig,
}

impl<T> StreamReader<T>
where
    T: Sample + Send + 'static,
{
    pub fn new(device: Device, config: StreamConfig, chunk_size: usize) -> Fallible<Self> {
        let back_buf = Arc::new(Mutex::new(Vec::with_capacity(2 * chunk_size)));
        let front_buf = Vec::with_capacity(2 * chunk_size);
        let chunk_buf = Vec::with_capacity(2 * chunk_size);

        let back_buf_worker = back_buf.clone();

        let stream = device.build_input_stream(
            &config,
            move |data: &[T], _: &cpal::InputCallbackInfo| {
                back_buf_worker.lock().unwrap().extend_from_slice(data)
            },
            move |err| println!("Audio error: {}", err),
        )?;

        stream.play().expect("Failed playing stream");

        Ok(Self {
            chunk_size,
            back_buf,
            front_buf,
            chunk_buf,
            chunk_offset: 0,
            _stream: stream,
            config,
        })
    }

    pub fn next_chunk(&mut self) -> Option<&mut [T]> {
        let leftover = self.chunk_buf.len().saturating_sub(self.chunk_offset);

        // If we still have enough data for a chunk, return it and advance the offset
        if leftover > self.chunk_size {
            let next_chunk_offset = self.chunk_offset + self.chunk_size;
            let chunk = &mut self.chunk_buf[self.chunk_offset..next_chunk_offset];

            self.chunk_offset = next_chunk_offset;

            return Some(chunk);
        }

        let mut swapped = false;
        {
            let mut back_buf = self.back_buf.lock().unwrap();
            if leftover + back_buf.len() >= self.chunk_size {
                mem::swap(&mut *back_buf, &mut self.front_buf);
                swapped = true;
            }
        }

        if swapped {
            // Shift the leftover data to the start of the buffer
            if leftover > 0 {
                self.chunk_buf.copy_within(self.chunk_offset.., 0);
            }
            self.chunk_buf.truncate(leftover);

            // Copy in new data, freeing in the front buffer to be swapped
            self.chunk_buf.extend_from_slice(&self.front_buf);
            self.front_buf.clear();

            // Reset the offset and return the first chunk in the buffer
            self.chunk_offset = self.chunk_size;

            return Some(&mut self.chunk_buf[0..self.chunk_size]);
        }

        None
    }

    pub fn config(&self) -> &StreamConfig {
        &self.config
    }

    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }
}

pub struct ConvertingStreamReader<T, U> {
    pub reader: StreamReader<T>,
    pub buf: Vec<U>,
}

impl<T, U> ConvertingStreamReader<T, U>
where
    T: Sample + Send + 'static,
    U: Sample,
{
    pub fn new(device: Device, config: StreamConfig, chunk_size: usize) -> Fallible<Self> {
        Ok(Self {
            reader: StreamReader::new(device, config, chunk_size)?,
            buf: vec![U::from(&0.0); chunk_size],
        })
    }

    pub fn next_chunk(&mut self) -> Option<&mut [U]> {
        if let Some(chunk) = self.reader.next_chunk() {
            for (out, sample) in self.buf.iter_mut().zip_eq(chunk.iter_mut()) {
                *out = U::from(sample);
            }

            Some(&mut self.buf)
        } else {
            None
        }
    }

    pub fn config(&self) -> &StreamConfig {
        self.reader.config()
    }

    pub fn chunk_size(&self) -> usize {
        self.reader.chunk_size()
    }
}
