use crate::{game::game::Game, message::Message};

use iced::{
    Application, Command, Element, Length, Subscription,
    time::{self, Duration},
    widget::{button, column, container, mouse_area, row, text},
};

pub struct Minesweaper {
    pub game: Game,
    pub time_elapsed: u32,
}

impl Application for Minesweaper {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                game: Game::new(20, 6),
                time_elapsed: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Minesweaper")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::CellLeftClick(x, y) => {
                self.game.reveal_cell(x, y);
            }

            Message::CellRightClick(x, y) => {
                self.game.flag_cell(x, y);
            }

            Message::Restart => {
                self.game = Game::new(6, 6);
                self.time_elapsed = 0;
            }

            Message::Tick => {
                self.time_elapsed += 1;
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.game.is_game_started && !self.game.is_game_over {
            time::every(Duration::from_secs(1)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn view(&self) -> Element<Message> {
        let mut board = column![];

        for y in 0..self.game.board.height {
            let mut current_row = row![];

            for x in 0..self.game.board.width {
                let cell = self.game.board.get_cell(x, y).unwrap();

                let label = if cell.is_revealed {
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

                let widget = mouse_area(
                    container(text(label))
                        .width(30)
                        .height(30)
                        .center_x()
                        .center_y(),
                )
                .on_press(Message::CellLeftClick(x, y))
                .on_right_press(Message::CellRightClick(x, y));

                current_row = current_row.push(widget);
            }

            board = board.push(current_row);
        }

        // Nombre de bombes restantes
        let remaining = self.game.board.mines_placed
            - self
                .game
                .board
                .cells
                .iter()
                .flatten()
                .filter(|c| c.is_flagged)
                .count();

        let status = if self.game.is_game_over {
            if self.game.is_game_lost {
                "Game Over"
            } else {
                "Victory!"
            }
        } else if self.game.is_game_started {
            "Playing"
        } else {
            "Ready"
        };

        let header = container(
            row![
                text(format!("(') {}", remaining)).size(28),
                button("Restart").on_press(Message::Restart),
                text(format!("Timer {} s", self.time_elapsed)).size(28),
            ]
            .spacing(40),
        )
        .width(Length::Fill)
        .center_x();

        let content = column![header, text(status).size(24), board,]
            .spacing(20)
            .align_items(iced::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
