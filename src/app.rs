use crate::{game::game::Game, message::Message};

use iced::{
    Element, Length, Sandbox, widget::{ column, container, mouse_area, row, text},
};

pub struct Minesweaper {
    pub game: Game,
}

impl Sandbox for Minesweaper {
    type Message = Message;

    fn new() -> Self {
        Self {
            game: Game::new(6, 6),
        }
    }

    fn title(&self) -> String {
        String::from("Minesweaper")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CellLeftClick(x, y) => {
                self.game.reveal_cell(x, y);
            }

            Message::CellRightClick(x, y) => {
                print!("Right click on cell ({}, {})\n", x, y);
                self.game.flag_cell(x, y);
            }

            Message::Restart => {}
        }
    }
    fn view(&self) -> Element<'_, Message> {
    let mut board = column![];

    for y in 0..self.game.board.height {
        let mut row = row![];

        for x in 0..self.game.board.width {
            let cell = self.game.board.get_cell(x, y).unwrap();

            let cell_text = if cell.is_revealed {
                if cell.is_mine {
                    "(`)".to_string()
                } else if cell.adjacent_mines > 0 {
                    cell.adjacent_mines.to_string()
                } else {
                    " ".to_string()
                }
            } else if cell.is_flagged {
                "|>".to_string()
            } else {
                "🟩".to_string()
            };

            let cell = mouse_area(
                container(text(cell_text))
                    .width(30)
                    .height(30)
                    .center_x()
                    .center_y(),
            )
            .on_press(Message::CellLeftClick(x, y))
            .on_right_press(Message::CellRightClick(x, y));

            row = row.push(cell);
        }

        board = board.push(row);
    }

    // Détermination de l'état de la partie
    let gamestate = if self.game.is_game_over && self.game.is_game_lost {
        "Game Over"
    } else if self.game.is_game_over && !self.game.is_game_lost {
        "You Won!"
    } else if self.game.is_game_started && !self.game.is_game_over {
        "Playing"
    } else {
        "Not Started"
    };

    // On regroupe le texte et le plateau dans une seule colonne
    let content = column![
        text(gamestate),
        board,
    ];

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}
}
