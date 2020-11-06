use std::{
    collections::VecDeque,
    fmt::{self, Display, Formatter},
    io::{self, Write},
    mem,
    sync::{Arc, Mutex},
};

use termion::{clear, color, cursor};

use crate::{constants::CONSTS, ui::UiBase, update_stat::UpdateStat};

struct LogLine {
    level: log::Level,
    text: String,
}

impl Display for LogLine {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let color: &'static dyn color::Color = match self.level {
            log::Level::Error => &color::Red,
            log::Level::Warn => &color::Yellow,
            log::Level::Info => &color::Cyan,
            log::Level::Debug => &color::Magenta,
            log::Level::Trace => &color::White,
        };

        write!(
            f,
            "{}{}{}",
            color::Fg(color),
            &self.text,
            color::Fg(color::Reset)
        )
    }
}

struct Padded<'a> {
    body: &'a str,
    padding: &'a str,
    width: usize,
}

impl<'a> Padded<'a> {
    fn new(body: &'a str, padding: &'a str, width: usize) -> Self {
        assert!(width >= body.len());
        assert!(padding.len() > 0);

        Self {
            body,
            padding,
            width,
        }
    }
}

impl<'a> Display for Padded<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let pad_n = self.width - self.body.chars().count();

        let pad_r = pad_n / 2;
        let pad_l = pad_n - pad_r;

        let padding_n = self.padding.chars().count();

        let pad_l_i = pad_l / padding_n;
        let pad_l_r = pad_l % padding_n;

        let pad_r_i = pad_r / padding_n;
        let pad_r_r = pad_r % padding_n;

        for _ in 0..pad_l_i {
            write!(f, "{}", &self.padding)?;
        }

        if pad_l_r > 0 {
            write!(
                f,
                "{}",
                &self.padding[..self.padding.char_indices().nth(pad_l_r).unwrap().0]
            )?;
        }

        write!(f, "{}", &self.body)?;

        for _ in 0..pad_r_i {
            write!(f, "{}", &self.padding)?;
        }

        if pad_r_r > 0 {
            write!(
                f,
                "{}",
                &self.padding[..self.padding.char_indices().nth(pad_r_r).unwrap().0]
            )?;
        }

        Ok(())
    }
}

struct ValueTable<'a> {
    rows: &'a [(&'a str, f64)],
}

impl<'a> ValueTable<'a> {
    pub fn new(rows: &'a [(&'a str, f64)]) -> Self {
        Self { rows }
    }
}

impl<'a> Display for ValueTable<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let label_width = self
            .rows
            .iter()
            .map(|(l, _)| l.chars().count())
            .max()
            .unwrap();

        let bar_width = CONSTS.console_width - label_width - 2;

        for (label, value) in self.rows.iter() {
            write!(f, "{:width$}: ", label, width = label_width)?;

            let value_scaled = value.max(0.0).min(1.0) * bar_width as f64;
            let value_fract = value_scaled.fract();
            let value_int = (value_scaled - value_fract) as usize;

            for _ in 0..value_int {
                write!(f, "█")?;
            }

            if value_int < bar_width {
                write!(f, "{}", frac_block(value_fract))?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

const FRAC_BLOCK_CHARS: [char; 9] = [' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];

fn frac_block(v: f64) -> char {
    let n = FRAC_BLOCK_CHARS.len();

    char::from(
        *FRAC_BLOCK_CHARS
            .iter()
            .enumerate()
            .find(|(i, _)| v < *i as f64 / (n - 1) as f64 + 1.0 / ((n - 1) * 2) as f64)
            .unwrap()
            .1,
    )
}

struct Logs {
    lines: VecDeque<LogLine>,
}

impl Logs {
    fn new() -> Self {
        Self {
            lines: VecDeque::new(),
        }
    }

    fn clear(&mut self) {
        self.lines.clear()
    }
}

pub struct Ui {
    logs: Arc<Mutex<Logs>>,
    swap_logs: Logs,
    prev_update_stat: Option<UpdateStat>,
}

impl UiBase for Ui {
    fn new() -> Self {
        println!("{}{}", clear::All, cursor::Goto(1, 1));

        Self {
            logs: Arc::new(Mutex::new(Logs::new())),
            swap_logs: Logs::new(),
            prev_update_stat: None,
        }
    }

    fn log_output(&self) -> fern::Output {
        let logs = Arc::clone(&self.logs);
        fern::Output::call(move |record| log_record(&*logs, record))
    }

    fn draw(&mut self, update_stat: &UpdateStat) {
        self.swap_logs.clear();

        {
            // NOTE: LOGGING CRITICAL SECTION
            // Logs are locked here, do not call any logging functions or you WILL deadlock.
            let mut logs = self.logs.lock().unwrap();

            mem::swap(&mut *logs, &mut self.swap_logs);

            // NOTE: END LOGGING CRITICAL SECTION
        }

        let logs = &self.swap_logs;

        if logs.lines.is_empty() && Some(update_stat) == self.prev_update_stat.as_ref() {
            return;
        }

        let prev_update_stat = self.prev_update_stat.replace(update_stat.clone());

        let table_rows = [
            ("Activity", update_stat.activity_value),
            ("Alpha", update_stat.alpha_value),
            ("Local Similarity", update_stat.local_similarity_value),
            ("Global Similarity", update_stat.global_similarity_value),
        ];

        let height = 1 + table_rows.len();

        if prev_update_stat.is_some() {
            print!("{}", cursor::Left(CONSTS.console_width as u16));
            for _ in 0..height {
                print!("{}", cursor::Up(1));
                print!("{}", clear::CurrentLine);
            }
            std::io::stdout().lock().flush().unwrap();
        }

        for log in logs.lines.iter() {
            println!("{}", log);
        }

        println!("{}", Padded::new(" Heuristics ", "=", CONSTS.console_width));
        print!("{}", ValueTable::new(&table_rows));

        io::stdout().lock().flush().unwrap();
    }
}

fn log_record(logs: &Mutex<Logs>, record: &log::Record) {
    let text = format!("{}", record.args());

    {
        // NOTE: LOGGING CRITICAL SECTION
        // Logs are locked here, do not call any logging functions or you WILL deadlock.
        let mut logs = logs.lock().unwrap();

        for mut line in text.lines() {
            while !line.is_empty() {
                let n = line
                    .char_indices()
                    .map(|(i, _)| i)
                    .skip_while(|&i| i < CONSTS.console_width)
                    .next()
                    .unwrap_or(line.len());

                logs.lines.push_back(LogLine {
                    level: record.level(),
                    text: line[..n].to_string(),
                });

                line = &line[n..];
            }
        }

        // NOTE: END LOGGING CRITICAL SECTION
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padded() {
        let expected = "====Foo===";
        assert_eq!(&Padded::new("Foo", "=", 10).to_string(), expected);
    }

    #[test]
    fn test_padded_2() {
        let expected = "=+=+Foo=+=";
        assert_eq!(&Padded::new("Foo", "=+", 10).to_string(), expected);
    }
}
