use crate::styling;
use crate::Message;
use iced::alignment::{Horizontal, Vertical};
use iced::theme;
use iced::widget::button::Button;
use iced::widget::{button, text};
use iced::{Length, Renderer, Theme};

pub fn button_view(
    txt: &str,
    message: Message,
    w: u16,
    style: styling::Button,
) -> Button<'static, Message, Renderer<Theme>> {
    button(
        text(txt.to_string())
            .size(25)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .width(Length::FillPortion(w))
    .on_press(message)
    .padding([5, 10])
    .style(theme::Button::Custom(Box::new(style)))
}

pub fn buttons_collect<T: ToString>(
    nums: Vec<T>,
    style: styling::Button,
) -> Vec<Button<'static, Message, Renderer<Theme>>> {
    nums.into_iter()
        .map(|b| button_view(&b.to_string(), Message::OnInput(b.to_string()), 1, style))
        .collect()
}
