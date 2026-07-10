mod app;
mod game;
mod message;
mod counter;

use app::Minesweaper;
use counter::Counter;

use iced::{
    Alignment, Application, Element, Length, Sandbox, Settings, widget::{button, column, container, text},
};

pub fn main() -> iced::Result {
    Minesweaper::run(Settings::default())
    //Counter::run(Settings::default())
}


