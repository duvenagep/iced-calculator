use crate::styling;
use crate::Message;
use iced::alignment::{Horizontal, Vertical};
use iced::theme;
use iced::widget::button::Button;
use iced::widget::{button, text};
use iced::{Length, Renderer, Theme};

pub fn button_view(
    txt: String,
    message: Message,
    w: u16,
    style: styling::Button,
) -> Button<'static, Message, Renderer<Theme>> {
    button(
        text(format!("{txt}"))
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
        .map(|b| button_view(b.to_string(), Message::OnInput(b.to_string()), 1, style))
        .collect()
}

// let _zero = view::button_view(
//     "0".to_string(),
//     Message::OnInput("0".to_string()),
//     2,
//     styling::Button::Num,
// );

// let _comma = view::button_view(
//     ",".to_string(),
//     Message::OnInput(",".to_string()),
//     1,
//     styling::Button::Num,
// );
// let numerics = view::buttons_collect((1..=9).collect(), styling::Button::Num);
// let arthimetics = view::buttons_collect(vec!['=', '+', '-', '÷'], styling::Button::Arth);
// let mathematics = view::buttons_collect(vec!['%', '+', '-', '÷'], styling::Button::Arth);
