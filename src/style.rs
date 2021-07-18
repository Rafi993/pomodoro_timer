use iced::{button, container, Background, Color};

const BG_COLOR: Color = Color::from_rgb(
    0x219 as f32 / 255.0,
    0x82 as f32 / 255.0,
    0x77 as f32 / 255.0,
);

const DARK_BG_COLOR: Color = Color::from_rgb(
    0x250 as f32 / 255.0,
    0x50 as f32 / 255.0,
    0x50 as f32 / 255.0,
);

pub struct Container;

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(BG_COLOR)),
            text_color: Some(Color::WHITE),
            ..container::Style::default()
        }
    }
}

pub struct ButtonStyle;

impl button::StyleSheet for ButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: iced::Vector::default(),
            background: Some(Background::Color(BG_COLOR)),
            text_color: Color::WHITE,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            shadow_offset: iced::Vector::default(),
            background: Some(Background::Color(DARK_BG_COLOR)),
            text_color: Color::WHITE,
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            shadow_offset: iced::Vector::default(),
            background: Some(Background::Color(DARK_BG_COLOR)),
            text_color: Color::WHITE,
            ..button::Style::default()
        }
    }
}
