use crate::{constants::CONSTS, UpdateStat};

#[cfg(unix)]
mod fancy;

mod simple;

pub trait UiBase {
    fn new() -> Self;
    fn log_output(&self) -> fern::Output;
    fn draw(&mut self, update_stat: &UpdateStat);
}

pub struct Ui(UiImpl);

enum UiImpl {
    #[cfg(unix)]
    Fancy(fancy::Ui),
    Simple(simple::Ui),
}

impl UiBase for Ui {
    #[cfg(unix)]
    fn new() -> Self {
        if CONSTS.fancy_terminal {
            Ui(UiImpl::Fancy(fancy::Ui::new()))
        } else {
            Ui(UiImpl::Simple(simple::Ui::new()))
        }
    }

    #[cfg(not(unix))]
    fn new() -> Self {
        if CONSTS.fancy_terminal {
            println!("WARNING: Fancy terminal not supported on this platform. Setting ignored.");
        };

        Ui(UiImpl::Simple(simple::Ui::new()))
    }

    fn log_output(&self) -> fern::Output {
        match &self.0 {
            #[cfg(unix)]
            UiImpl::Fancy(ui) => ui.log_output(),
            UiImpl::Simple(ui) => ui.log_output(),
        }
    }

    fn draw(&mut self, update_stat: &UpdateStat) {
        match &mut self.0 {
            #[cfg(unix)]
            UiImpl::Fancy(ui) => ui.draw(update_stat),
            UiImpl::Simple(ui) => ui.draw(update_stat),
        }
    }
}
