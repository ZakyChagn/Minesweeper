use crate::{
    game::game::Game,
    message::Message,
};

use iced::{
    Element,
    Length,
    Sandbox,
    widget::{column, button, text, row, container},
};


pub struct Minesweaper{
    pub game: Game,
}

impl Sandbox for Minesweaper {
    type Message = Message;

    fn new() -> Self {
        Self {
            game: Game::new(20, 20),
        }
    }

    fn title(&self) -> String {
        String::from("Minesweaper")
    }

    fn update(&mut self, message: Message) {
       match message {
            Message::CellPressed(x, y) => {
                println!("Cell pressed at ({}, {})", x, y);
                self.game.reveal_cell(x, y);
            }
            Message::Restart => {}
        }
    }
    fn view(&self) -> Element<Message>{
        let mut board = column![];
        for y in 0..self.game.board.height {
            let mut row = row![];
            for x in 0..self.game.board.width {
                let cell = self.game.board.get_cell(x, y).unwrap();
                let cell_text = if cell.is_revealed {
                    if cell.is_mine {
                        "💣".to_string()
                    } else {
                        if cell.adjacent_mines > 0 {
                            cell.adjacent_mines.to_string()
                        } else {
                            " ".to_string()
                        }
                    }
                } else if cell.is_flagged {
                    "🚩".to_string()
                } else {
                    "🟩".to_string()
                };
                let button = button(text(cell_text))
                    .width(Length:: Fill)
                    .height(Length:: Fill)
                    .on_press(Message::CellPressed(x, y));
                row = row.push(button);
            }
            board = board.push(row);
        }
        container(board)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
