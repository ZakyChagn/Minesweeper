mod app;
mod game;
mod message;
mod counter;

use app::Minesweaper;
use counter::Counter;

use iced::{
    Sandbox, Settings, Element,
    widget::{column, button, text, container},
    Alignment, Length,
};

pub fn main() -> iced::Result {
    Minesweaper::run(Settings::default())
    //Counter::run(Settings::default())
}


