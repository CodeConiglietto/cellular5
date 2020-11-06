use crate::{constants::CONSTS, ui::UiBase, update_stat::UpdateStat};

pub struct Ui;

impl UiBase for Ui {
    fn new() -> Self {
        Self
    }

    fn log_output(&self) -> fern::Output {
        fern::Output::stdout("\n")
    }

    fn draw(&mut self, update_stat: &UpdateStat) {
        println!("{:?}", update_stat);
    }
}
