//! Buttons style
use iced::color;
use iced::theme::Palette;
use iced::widget::{button, container, text};
use iced::{Background, Color, Theme, Vector};

#[derive(Clone, Copy)]
pub enum Button {
    Num,
    Arth,
    Math,
}

impl button::StyleSheet for Button {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match &self {
            Button::Arth => button::Appearance {
                background: Some(Background::Color(color!(0xF2, 0xA3, 0x3C).into())),
                text_color: color!(255, 255, 255),
                border_radius: 0.0.into(),
                border_color: color!(0, 0, 0),
                border_width: 0.8,
                ..button::Appearance::default()
            },
            Button::Num => button::Appearance {
                background: Some(Background::Color(color!(0x5F, 0x5F, 0x61).into())),
                text_color: color!(255, 255, 255),
                border_radius: 0.0.into(),
                border_color: color!(0, 0, 0),
                border_width: 0.8,
                ..button::Appearance::default()
            },
            Button::Math => button::Appearance {
                background: Some(Background::Color(color!(0x3F, 0x40, 0x41).into())),
                text_color: color!(255, 255, 255),
                border_radius: 0.0.into(),
                border_color: color!(0, 0, 0),
                border_width: 0.8,
                ..button::Appearance::default()
            },
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        match self {
            Button::Num => button::Appearance {
                background: Some(Background::Color(color!(0x5F, 0x5F, 0x60).into())),
                border_color: color!(5, 5, 5),
                shadow_offset: Vector::default(),
                ..self.active(style)
            },
            Button::Arth => button::Appearance {
                background: Some(Background::Color(color!(0x8E, 0x8E, 0x8F).into())),
                border_color: color!(5, 5, 5),
                border_width: 1.5,

                shadow_offset: Vector::default(),
                ..self.active(style)
            },
            Button::Math => button::Appearance {
                background: Some(Background::Color(color!(0x8E, 0x8E, 0x8F).into())),
                border_color: color!(5, 5, 5),
                border_width: 1.5,

                shadow_offset: Vector::default(),
                ..self.active(style)
            },
        }
    }
}

pub struct InputResultContainer(pub Palette);

impl container::StyleSheet for InputResultContainer {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: color!(255, 255, 255).into(),
            background: Some(Background::Color(color!(0x2A, 0x2B, 0x2C).into())),

            ..container::Appearance::default()
        }
    }
}
