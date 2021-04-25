use mutagen::Reborrow;

use crate::prelude::*;
use ggez::mint::Point2;

pub trait MutagenArg {
    fn depth(&self) -> usize;
    fn gamepads(&self) -> &Gamepads;
    fn mic_spectrograms(&self) -> &Option<FrequencySpectrograms>;
    fn camera_frames(&self) -> &Option<CameraFrames>;
}

pub struct GenArg<'a> {
    pub nodes: &'a mut [NodeSet],
    pub data: &'a mut DataSet,
    pub depth: usize,
    pub current_t: usize,
    pub coordinate_set: CoordinateSet,
    pub history: &'a History,
    pub mic_spectrograms: &'a Option<FrequencySpectrograms>,
    pub image_preloader: &'a Preloader<Image>,
    pub profiler: &'a mut Option<MutagenProfiler>,
    pub gamepads: &'a mut Gamepads,
    pub mouse_position: &'a mut Point2<f32>,
    pub camera_frames: &'a Option<CameraFrames>,
}

impl<'a, 'b: 'a> Reborrow<'a, 'b, GenArg<'a>> for GenArg<'b> {
    fn reborrow(&'a mut self) -> GenArg<'a> {
        GenArg {
            nodes: &mut self.nodes,
            data: &mut self.data,
            depth: self.depth,
            current_t: self.current_t,
            coordinate_set: self.coordinate_set,
            history: &self.history,
            mic_spectrograms: &self.mic_spectrograms,
            image_preloader: &self.image_preloader,
            profiler: &mut self.profiler,
            gamepads: &mut self.gamepads,
            mouse_position: &mut self.mouse_position,
            camera_frames: &self.camera_frames,
        }
    }
}

impl<'a> mutagen::State for GenArg<'a> {
    fn handle_event(&mut self, event: mutagen::Event) {
        if let Some(profiler) = &mut self.profiler {
            profiler.handle_event(event);
        }
    }
}

impl<'a> MutagenArg for GenArg<'a> {
    fn depth(&self) -> usize {
        self.depth.saturating_sub(1) // Subtract 1 since NodeBox adds 1 earlier than the mutagen code will see it
    }

    fn gamepads(&self) -> &Gamepads {
        &self.gamepads
    }

    fn mic_spectrograms(&self) -> &Option<FrequencySpectrograms> {
        &self.mic_spectrograms
    }

    fn camera_frames(&self) -> &Option<CameraFrames> {
        &self.camera_frames
    }
}

pub struct MutArg<'a> {
    pub nodes: &'a mut [NodeSet],
    pub data: &'a mut DataSet,
    pub depth: usize,
    pub current_t: usize,
    pub coordinate_set: CoordinateSet,
    pub history: &'a History,
    pub mic_spectrograms: &'a Option<FrequencySpectrograms>,
    pub image_preloader: &'a Preloader<Image>,
    pub profiler: &'a mut Option<MutagenProfiler>,
    pub gamepads: &'a mut Gamepads,
    pub mouse_position: &'a mut Point2<f32>,
    pub camera_frames: &'a Option<CameraFrames>,
}

impl<'a, 'b: 'a> Reborrow<'a, 'b, MutArg<'a>> for MutArg<'b> {
    fn reborrow(&'a mut self) -> MutArg<'a> {
        MutArg {
            nodes: &mut self.nodes,
            data: &mut self.data,
            depth: self.depth,
            current_t: self.current_t,
            coordinate_set: self.coordinate_set,
            history: &self.history,
            mic_spectrograms: &self.mic_spectrograms,
            image_preloader: &self.image_preloader,
            profiler: &mut self.profiler,
            gamepads: &mut self.gamepads,
            mouse_position: &mut self.mouse_position,
            camera_frames: &self.camera_frames,
        }
    }
}

impl<'a> From<MutArg<'a>> for GenArg<'a> {
    fn from(arg: MutArg<'a>) -> Self {
        Self {
            nodes: arg.nodes,
            data: arg.data,
            depth: arg.depth,
            current_t: arg.current_t,
            coordinate_set: arg.coordinate_set,
            history: arg.history,
            mic_spectrograms: arg.mic_spectrograms,
            image_preloader: arg.image_preloader,
            profiler: arg.profiler,
            gamepads: arg.gamepads,
            mouse_position: arg.mouse_position,
            camera_frames: arg.camera_frames,
        }
    }
}

impl<'a> mutagen::State for MutArg<'a> {
    fn handle_event(&mut self, event: mutagen::Event) {
        if let Some(profiler) = &mut self.profiler {
            profiler.handle_event(event);
        }
    }
}

impl<'a> MutagenArg for MutArg<'a> {
    fn depth(&self) -> usize {
        self.depth.saturating_sub(1) // Subtract 1 since NodeBox adds 1 earlier than the mutagen code will see it
    }

    fn gamepads(&self) -> &Gamepads {
        &self.gamepads
    }

    fn mic_spectrograms(&self) -> &Option<FrequencySpectrograms> {
        &self.mic_spectrograms
    }

    fn camera_frames(&self) -> &Option<CameraFrames> {
        &self.camera_frames
    }
}

#[derive(Clone)]
pub struct ComArg<'a> {
    pub nodes: &'a [NodeSet],
    pub data: &'a DataSet,
    pub coordinate_set: CoordinateSet,
    pub history: &'a History,
    pub depth: usize,
    pub current_t: usize,
    pub mic_spectrograms: &'a Option<FrequencySpectrograms>,
    pub gamepads: &'a Gamepads,
    pub mouse_position: &'a Point2<f32>,
    pub camera_frames: &'a Option<CameraFrames>,
}

impl<'a> ComArg<'a> {
    pub fn replace_coords(self, other: &SNPoint) -> Self {
        let mut new = self.clone();

        new.coordinate_set.x = other.x();
        new.coordinate_set.y = other.y();

        new
    }
    pub fn replace_coordinate_set(self, other: &CoordinateSet) -> Self {
        let mut new = self.clone();

        new.coordinate_set = other.clone();

        new
    }
}

impl<'a, 'b: 'a> Reborrow<'a, 'b, ComArg<'a>> for ComArg<'b> {
    fn reborrow(&'a mut self) -> ComArg<'a> {
        ComArg {
            nodes: &self.nodes,
            data: &self.data,
            coordinate_set: self.coordinate_set,
            history: &self.history,
            depth: self.depth,
            current_t: self.current_t,
            mic_spectrograms: &self.mic_spectrograms,
            gamepads: &self.gamepads,
            mouse_position: &self.mouse_position,
            camera_frames: &self.camera_frames,
        }
    }
}

impl<'a> mutagen::State for ComArg<'a> {}

impl<'a> MutagenArg for ComArg<'a> {
    fn depth(&self) -> usize {
        self.depth.saturating_sub(1) // Subtract 1 since NodeBox adds 1 earlier than the mutagen code will see it
    }

    fn gamepads(&self) -> &Gamepads {
        &self.gamepads
    }

    fn mic_spectrograms(&self) -> &Option<FrequencySpectrograms> {
        &self.mic_spectrograms
    }

    fn camera_frames(&self) -> &Option<CameraFrames> {
        &self.camera_frames
    }
}

pub struct UpdArg<'a> {
    pub nodes: &'a mut [NodeSet],
    pub data: &'a mut DataSet,
    pub coordinate_set: CoordinateSet,
    pub history: &'a History,
    pub depth: usize,
    pub current_t: usize,
    pub image_preloader: &'a Preloader<Image>,
    pub mic_spectrograms: &'a Option<FrequencySpectrograms>,
    pub profiler: &'a mut Option<MutagenProfiler>,
    pub gamepads: &'a mut Gamepads,
    pub mouse_position: &'a mut Point2<f32>,
    pub camera_frames: &'a Option<CameraFrames>,
}

impl<'a, 'b: 'a> Reborrow<'a, 'b, UpdArg<'a>> for UpdArg<'b> {
    fn reborrow(&'a mut self) -> UpdArg<'a> {
        UpdArg {
            nodes: &mut self.nodes,
            data: &mut self.data,
            coordinate_set: self.coordinate_set,
            history: &self.history,
            depth: self.depth,
            current_t: self.current_t,
            mic_spectrograms: &self.mic_spectrograms,
            image_preloader: &self.image_preloader,
            profiler: &mut self.profiler,
            gamepads: &mut self.gamepads,
            mouse_position: &mut self.mouse_position,
            camera_frames: &self.camera_frames,
        }
    }
}

impl<'a> From<UpdArg<'a>> for ComArg<'a> {
    fn from(arg: UpdArg<'a>) -> Self {
        Self {
            nodes: arg.nodes,
            data: arg.data,
            coordinate_set: arg.coordinate_set,
            history: arg.history,
            depth: arg.depth,
            current_t: arg.current_t,
            mic_spectrograms: arg.mic_spectrograms,
            gamepads: arg.gamepads,
            mouse_position: arg.mouse_position,
            camera_frames: arg.camera_frames,
        }
    }
}

impl<'a> From<GenArg<'a>> for UpdArg<'a> {
    fn from(arg: GenArg<'a>) -> Self {
        Self {
            nodes: arg.nodes,
            data: arg.data,
            depth: arg.depth,
            current_t: arg.current_t,
            coordinate_set: arg.coordinate_set,
            history: arg.history,
            image_preloader: arg.image_preloader,
            profiler: arg.profiler,
            mic_spectrograms: arg.mic_spectrograms,
            gamepads: arg.gamepads,
            mouse_position: arg.mouse_position,
            camera_frames: arg.camera_frames,
        }
    }
}

impl<'a> mutagen::State for UpdArg<'a> {
    fn handle_event(&mut self, event: mutagen::Event) {
        if let Some(profiler) = &mut self.profiler {
            profiler.handle_event(event);
        }
    }
}

impl<'a> MutagenArg for UpdArg<'a> {
    fn depth(&self) -> usize {
        self.depth.saturating_sub(1) // Subtract 1 since NodeBox adds 1 earlier than the mutagen code will see it
    }

    fn gamepads(&self) -> &Gamepads {
        &self.gamepads
    }

    fn mic_spectrograms(&self) -> &Option<FrequencySpectrograms> {
        &self.mic_spectrograms
    }

    fn camera_frames(&self) -> &Option<CameraFrames> {
        &self.camera_frames
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UpdateState<'a> {
    //the set of coordinates for the update
    pub coordinate_set: CoordinateSet,
    //cell array to read from
    pub history: &'a History,
}

impl<'a> From<GenArg<'a>> for () {
    fn from(_arg: GenArg<'a>) -> Self {}
}

impl<'a> From<MutArg<'a>> for () {
    fn from(_arg: MutArg<'a>) -> Self {}
}

impl<'a> From<UpdArg<'a>> for () {
    fn from(_arg: UpdArg<'a>) -> Self {}
}
