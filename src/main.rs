use iced::widget::{column, text, text_input, scrollable};
use iced::{widget, Renderer, Element, Sandbox, Settings};
mod typing_area;
fn main() {
    let _ = TextDisplay::run(Settings::default());
}

#[derive(Debug, Clone)]
enum Message {
    Path(String),
}

struct TextDisplay {
    text: String,
    path_text: String,
}

impl Sandbox for TextDisplay {
    type Message = Message;
    fn new() -> Self {
        Self {
            text: String::new(),
            path_text: String::new(),
        }
    }
    fn title(&self) -> String {
        String::from("typeracer")
    }
    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Path(s) => {
                self.path_text = s.clone();
                match std::fs::read(s) {
                    Ok(b) => match String::from_utf8(b) {
                        Ok(s) => self.text = s,
                        Err(_) => {
                            self.text = "file contents are not valid utf8".to_string();
                        }
                    },
                    Err(_) => {
                        self.text = "file cannot be read".to_string();
                    }
                }
            }
        }
    }
    fn view(&self) -> Element<Message> {
        column![
            text_input("path to file", self.path_text.as_str()).on_input(|s| Message::Path(s)),
            text(self.text.as_str()),
        ]
        .into()
    }
}
