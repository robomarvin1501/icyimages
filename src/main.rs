use iced::{
    widget::{container, Container, Image},
    Element, Length,
};

fn main() -> iced::Result {
    iced::run("Icyimage", IcyImage::update, IcyImage::view)
}

#[derive(Default)]
struct IcyImage {}

impl IcyImage {
    fn update(&mut self, message: Message) {
        match message {
            Message::Next => {}
            Message::Previous => {}
        }
    }

    fn view(&self) -> Element<Message> {
        let img = Image::new("resources/release_party.jpg")
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
