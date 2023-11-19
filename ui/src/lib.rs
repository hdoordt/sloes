use iced::{Application, Settings};

mod components;
mod sluus_ui;
mod types;

use sluus_ui::SluusUi;

pub fn run_it() -> iced::Result {
    SluusUi::run(Settings::default())
}
