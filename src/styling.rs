//! Buttons style
use iced::theme::Palette;
use iced::widget::{button, container};
use iced::{color, Border};
use iced::{Background, Theme};

#[derive(Clone, Copy)]
pub enum Button {
    Num,
    Arth,
    Math,
}

impl button::StyleSheet for Button {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        match &self {
            Button::Num => button::Appearance {
                background: Some(Background::Color(color!(0x5F, 0x5F, 0x61))),
                text_color: color!(255, 255, 255),
                border: Border {
                    radius: 0.0.into(),
                    color: color!(25, 25, 25),
                    width: 0.5,
                },
                ..button::Appearance::default()
            },
            Button::Arth => button::Appearance {
                background: Some(Background::Color(color!(0xF2, 0xA3, 0x3C))),
                text_color: color!(255, 255, 255),
                border: Border {
                    radius: 0.0.into(),
                    color: color!(25, 25, 25),
                    width: 0.5,
                },
                ..button::Appearance::default()
            },
            Button::Math => button::Appearance {
                background: Some(Background::Color(color!(0x3F, 0x40, 0x41))),
                text_color: color!(255, 255, 255),
                border: Border {
                    radius: 0.0.into(),
                    color: color!(25, 25, 25),
                    width: 0.5,
                },
                ..button::Appearance::default()
            },
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        match self {
            Button::Num => button::Appearance {
                background: Some(Background::Color(color!(0xA1, 0xA2, 0xA4))),
                ..self.active(style)
            },
            Button::Arth => button::Appearance {
                background: Some(Background::Color(color!(0xC0, 0x81, 0x2D))),
                ..self.active(style)
            },
            Button::Math => button::Appearance {
                background: Some(Background::Color(color!(0x5F, 0x5F, 0x61))),
                ..self.active(style)
            },
        }
    }
}

pub struct InputResultContainer(pub Palette);

impl container::StyleSheet for InputResultContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: color!(255, 255, 255).into(),
            background: Some(Background::Color(color!(0x29, 0x29, 0x26))),

            ..container::Appearance::default()
        }
    }
}
