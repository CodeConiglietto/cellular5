use crate::{prelude::*, ui::UiBase, update_stat::UpdateStat};

pub struct Ui;

impl UiBase for Ui {
    fn new() -> Self {
        Self
    }

    fn log_output(&self) -> fern::Output {
        fern::Output::stdout("\n")
    }

    fn draw(&mut self, update_stat: &UpdateStat, _gamepads: &Gamepads) {
        println!("{:#?}", update_stat);
    }
}
