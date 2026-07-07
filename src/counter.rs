use iced::{
    Sandbox, Settings, Element,
    widget::{column, button, text, container},
    Alignment, Length,
};

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
}

pub struct Counter {
    pub value: i32,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Compteur Iced 0.12")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }

    fn view(&self) -> Element<'_,Message> {
        // Layout vertical centré
        let content = column![
            button("+").on_press(Message::Increment),
            text(self.value).size(40),
            button("-").on_press(Message::Decrement),
        ]
        .spacing(20)                     // espace entre les widgets
        .align_items(Alignment::Center); // centre horizontalement

        // Container qui centre verticalement et horizontalement
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            
            .into()
    }
}