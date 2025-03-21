use iced::executor;
use iced::keyboard::{Event as KeyEvent, Key};
use iced::widget::{button, column, image, row, text};
use iced::{Application, Command, Element, Event, Settings, Subscription, Theme};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

struct ImageViewer {
    images: Vec<PathBuf>,
    current_index: usize,
}

#[derive(Debug, Clone)]
enum Message {
    Next,
    Previous,
    KeyPressed(Key),
}

impl Application for ImageViewer {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Vec<PathBuf>;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let images = flags;
        (
            Self {
                images,
                current_index: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Image Viewer".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Next => {
                if self.current_index + 1 < self.images.len() {
                    self.current_index += 1;
                }
            }
            Message::Previous => {
                if self.current_index > 0 {
                    self.current_index -= 1;
                }
            }
            Message::KeyPressed(key) => match key {
                Key::Character(ch) if ch == "n" => return self.update(Message::Next),
                Key::Character(ch) if ch == "p" => return self.update(Message::Previous),
                _ => {}
            },
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let img = if self.images.is_empty() {
            text("No images found").into()
        } else {
            image(self.images[self.current_index].to_str().unwrap()).into()
        };

        column![
            img,
            row![
                button("Previous").on_press(Message::Previous),
                button("Next").on_press(Message::Next),
            ]
        ]
        .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::subscription::events().map(|event| {
            if let Event::Keyboard(KeyEvent::KeyPressed { key, .. }) = event {
                Message::KeyPressed(key)
            } else {
                Message::Next // No-op
            }
        })
    }
}

fn main() -> iced::Result {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <image_or_directory>", args[0]);
        return Ok(());
    }

    let path = Path::new(&args[1]);
    let images = if path.is_dir() {
        fs::read_dir(path)
            .unwrap()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()?.to_str()?.to_lowercase() == "png"
                    || path.extension()?.to_str()?.to_lowercase() == "jpg"
                    || path.extension()?.to_str()?.to_lowercase() == "jpeg"
                {
                    Some(path)
                } else {
                    None
                }
            })
            .collect()
    } else if path.is_file() {
        vec![path.to_path_buf()]
    } else {
        eprintln!("Invalid path provided.");
        return Ok(());
    };

    ImageViewer::run(Settings::with_flags(images))
}
