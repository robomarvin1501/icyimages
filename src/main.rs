use iced::{
    widget::{container, Container, Image},
    Element, Length,
};

fn main() -> iced::Result {
    iced::run("Icyimage", IcyImage::update, IcyImage::view)
}

#[derive(Default)]
struct IcyImage {
    paths: Vec<String>,
    current_index: usize,
}

impl IcyImage {
    fn new(flags: Vec<String>) -> Self {
        let images = flags;
        Self {
            paths: images,
            current_index: 0,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Next => {
                if self.current_index < self.paths.len() {
                    self.current_index += 1;
                }
            }
            Message::Previous => {
                if self.current_index > 0 {
                    self.current_index -= 1;
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let img = Image::new(self.paths.get(self.current_index).unwrap())
            .width(Length::Fill)
            .height(Length::Fill);
        Container::new(img)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(container::dark)
            .into()
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Next,
    Previous,
}
