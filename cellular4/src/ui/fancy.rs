use std::{
    collections::VecDeque,
    fmt::{self, Display, Formatter},
    io::{self, Write},
    mem,
    sync::{Arc, Mutex},
};

use termion::{clear, color, cursor};

use crate::{prelude::*, ui::UiBase, update_stat::UpdateStat};

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
        assert!(!padding.is_empty());

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
    width: usize,
    rows: &'a [(&'a str, f64)],
}

impl<'a> ValueTable<'a> {
    pub fn new(width: usize, rows: &'a [(&'a str, f64)]) -> Self {
        Self { width, rows }
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

        let bar_width = self.width - label_width - 2;

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

                for _ in (value_int + 1)..bar_width {
                    write!(f, " ")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

const FRAC_BLOCK_CHARS: [char; 9] = [' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];

fn frac_block(v: f64) -> char {
    let n = FRAC_BLOCK_CHARS.len();

    *FRAC_BLOCK_CHARS
        .iter()
        .enumerate()
        .find(|(i, _)| v < *i as f64 / (n - 1) as f64 + 1.0 / ((n - 1) * 2) as f64)
        .unwrap()
        .1
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

    fn draw(&mut self, update_stat: &UpdateStat, gamepads: &Gamepads) {
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

        let prev_update_stat = self.prev_update_stat.replace(*update_stat);

        let table_rows = [
            ("Activity", update_stat.activity_value),
            ("Alpha", update_stat.alpha_value),
            ("Local Similarity", update_stat.local_similarity_value),
            ("Global Similarity", update_stat.global_similarity_value),
            ("Graph Stability", update_stat.graph_stability),
        ];

        let height = if gamepads.gamepads.is_empty() {
            1 + table_rows.len()
        } else {
            1 + usize::max(table_rows.len(), GAMEPAD_DISPLAY_HEIGHT)
        };

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

        // TODO Refactor this if it gets any more complex
        let table_width = CONSTS.console_width - gamepads.gamepads.len() * GAMEPAD_DISPLAY_WIDTH;
        println!("{}", Padded::new(" Heuristics ", "=", CONSTS.console_width));
        print!("{}", ValueTable::new(table_width, &table_rows));

        if !gamepads.gamepads.is_empty() && table_rows.len() < GAMEPAD_DISPLAY_HEIGHT {
            for _ in 0..(GAMEPAD_DISPLAY_HEIGHT - table_rows.len()) {
                for _ in 0..table_width {
                    print!(" ");
                }
                println!();
            }
        }

        for (i, gamepad) in gamepads.gamepads.iter().enumerate() {
            println!(
                "{}{}{}",
                cursor::Up(GAMEPAD_DISPLAY_HEIGHT as u16),
                cursor::Right((table_width + i * GAMEPAD_DISPLAY_WIDTH) as u16),
                GamepadDisplay(gamepad)
            );
        }

        io::stdout().lock().flush().unwrap();
    }
}

const GAMEPAD_DISPLAY_HEIGHT: usize = 7;
const GAMEPAD_DISPLAY_WIDTH: usize = 14;

struct GamepadDisplay<'a>(&'a Gamepad);

impl<'a> Display for GamepadDisplay<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let bts = &self.0.button_states;
        let axs = &self.0.axis_states;

        // TODO Add LeftZ and RightZ axes when working
        let l2 = use_color(bts.get(GamepadButton::LeftTrigger2).in_use);
        let r2 = use_color(bts.get(GamepadButton::RightTrigger2).in_use);

        let l1 = use_color(bts.get(GamepadButton::LeftTrigger).in_use);
        let r1 = use_color(bts.get(GamepadButton::RightTrigger).in_use);

        let lx = use_color(axs.get(GamepadAxis::LeftStickX).in_use);
        let ly = use_color(axs.get(GamepadAxis::LeftStickY).in_use);
        let l3 = use_color(bts.get(GamepadButton::LeftThumb).in_use);

        let du =
            use_color(bts.get(GamepadButton::DPadUp).in_use || axs.get(GamepadAxis::DPadY).in_use);
        let dl = use_color(
            bts.get(GamepadButton::DPadLeft).in_use || axs.get(GamepadAxis::DPadX).in_use,
        );
        let dr = use_color(
            bts.get(GamepadButton::DPadRight).in_use || axs.get(GamepadAxis::DPadX).in_use,
        );
        let dd = use_color(
            bts.get(GamepadButton::DPadDown).in_use || axs.get(GamepadAxis::DPadY).in_use,
        );

        let rx = use_color(axs.get(GamepadAxis::RightStickX).in_use);
        let ry = use_color(axs.get(GamepadAxis::RightStickY).in_use);
        let r3 = use_color(bts.get(GamepadButton::RightThumb).in_use);

        let n = use_color(bts.get(GamepadButton::North).in_use);
        let w = use_color(bts.get(GamepadButton::West).in_use);
        let e = use_color(bts.get(GamepadButton::East).in_use);
        let s = use_color(bts.get(GamepadButton::South).in_use);

        let c = color::Fg(color::Reset);
        let l = cursor::Left(GAMEPAD_DISPLAY_WIDTH as u16);
        let d = cursor::Down(1);

        write!(f, r#"  {}Π{}        {}Π{}{}{}  "#, l2, c, r2, c, l, d)?;
        write!(f, r#" /{}Π{}\______/{}Π{}\{}{} "#, l1, c, r1, c, l, d)?;
        write!(f, r#"| {}|{}        {}o{} |{}{}"#, ly, c, n, c, l, d)?;
        write!(
            f,
            r#"|{}-{}{}O{}{}-{} {}|{}  {}|{} {}o{} {}o{}|{}{}"#,
            lx, c, l3, c, lx, c, du, c, ry, c, w, c, e, c, l, d
        )?;
        write!(
            f,
            r#"| {}|{} {}-{} {}-{}{}-{}{}O{}{}-{} {}o{} |{}{}"#,
            ly, c, dl, c, dr, c, rx, c, r3, c, rx, c, s, c, l, d
        )?;
        write!(f, r#"|   _{}|{}__{}|{}_   |{}{}"#, dd, c, ry, c, l, d)?;
        write!(f, r#"|__|      |__|"#)?;

        Ok(())
    }
}

fn use_color(in_use: bool) -> color::Fg<&'static dyn color::Color> {
    if in_use {
        color::Fg(&color::Green)
    } else {
        color::Fg(&color::White)
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
                    .find(|&i| i >= CONSTS.console_width)
                    .unwrap_or_else(|| line.len());

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
