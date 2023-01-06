use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};

#[cfg(feature = "palette")]
use palette::rgb::{Srgb, Srgba};

/// A color in the sRGB color space.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color {
    /// Red component, 0.0 - 1.0
    pub r: f32,
    /// Green component, 0.0 - 1.0
    pub g: f32,
    /// Blue component, 0.0 - 1.0
    pub b: f32,
    /// Transparency, 0.0 - 1.0
    pub a: f32,
}

pub struct Appearance {
    pub background_color: Color::from_rgb(0.0, 0.0, 0.0),
    pub text_color: Color,
}

pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}


struct Hello;

impl Application for Hello {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Hello, Command<Self::Message>) {
        (Hello, Command::none())
    }

    fn title(&self) -> String {
        String::from("myNotes")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        "Hello, world!".into()
    }

    
}
