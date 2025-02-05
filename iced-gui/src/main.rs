use iced::Center;
use iced::widget::{button, text, Column};

#[derive(Default)]
struct Counter {
    count: i32,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.count += 1;
            }
            Message::Decrement => {
                self.count -= 1;
            }
        }
    }

    fn view(&self) -> Column<Message> {
        iced::widget::column![
            button("Increment").on_press(Message::Increment),
            text(self.count).size(50),
            button("Decrement").on_press(Message::Decrement),
        ].padding(20).align_x(Center)
    }
}

fn main() -> iced::Result {
    iced::run("katabasis::mod_manager", Counter::update, Counter::view)
}
