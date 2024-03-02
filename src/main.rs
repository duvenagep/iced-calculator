#![warn(clippy::all)]

mod styling;
mod view;

use iced::alignment::{Horizontal, Vertical};
use iced::keyboard::{key, Key};
use iced::theme;
use iced::widget::{button, column, container, row, text, Button, Column, Row};
use iced::window::settings::PlatformSpecific;
use iced::window::Position;
use iced::Size;
use iced::{executor, keyboard};
use iced::{window, Application, Command, Length, Settings, Theme};
use view::button_view;

fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: Size::new(232.0, 321.0),
            resizable: true,
            decorations: true,
            position: Position::Default,
            min_size: Some(Size::new(232.0, 321.0)),
            max_size: Some(Size::new(232.0, 321.0)),
            visible: true,
            transparent: true,
            platform_specific: PlatformSpecific {
                #[cfg(target_os = "macos")]
                title_hidden: true,

                #[cfg(target_os = "macos")]
                titlebar_transparent: true,

                #[cfg(target_os = "macos")]
                fullsize_content_view: true,
            },
            ..Default::default()
        },
        antialiasing: true,
        ..Default::default()
    };

    Calculator::run(settings)
}

// #[derive(Debug, Clone)]
// enum NumricInput<T> {
//     Decimal(T),
//     Whole(T),
// }

#[derive(Debug, Clone, Default)]
struct Calculator {
    lhs: String,
    rhs: String,
    operator: Option<char>,
    result: f64,
}

#[derive(Debug, Clone)]
pub enum Message {
    OnInput(String),
    Add,
    Subtract,
    Multiply,
    Devide,
    Answer,
    Clear,
}

impl Application for Calculator {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::new()
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Add => self.operator = Some('+'),
            Message::Subtract => self.operator = Some('-'),
            Message::Multiply => self.operator = Some('x'),
            Message::Devide => self.operator = Some('÷'),
            Message::Clear => {
                *self = Self::default();
            }
            Message::OnInput(val) => {
                if val == "b".to_string() && self.operator.is_none() {
                    self.lhs.pop();
                } else if val == "b".to_string() && !self.operator.is_none() {
                    self.rhs.pop();
                } else if self.operator.is_none() {
                    self.lhs.push_str(&val);
                } else {
                    self.rhs.push_str(&val);
                }
            }

            Message::Answer => {
                let l = if self.lhs.contains(",") {
                    self.lhs.replace(",", ".").parse::<f64>().unwrap()
                } else {
                    self.lhs.push_str(".0");
                    self.lhs.parse::<f64>().unwrap()
                };
                let r = if self.rhs.contains(",") {
                    self.rhs.replace(",", ".").parse::<f64>().unwrap()
                } else {
                    self.rhs.push_str(".0");
                    self.rhs.parse::<f64>().unwrap()
                };

                match self.operator {
                    Some('+') => {
                        self.result = l + r;
                    }
                    Some('-') => {
                        self.result = l - r;
                    }
                    Some('x') => {
                        self.result = l * r;
                    }
                    Some('÷') => {
                        self.result = l / r;
                    }
                    Some(_) | None => {}
                }
            }
        }
        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        keyboard::on_key_press(|key, modifiers| match key.as_ref() {
            key::Key::Character("0") => Some(Message::OnInput("0".to_string())),
            key::Key::Character("1") => Some(Message::OnInput("1".to_string())),
            key::Key::Character("2") => Some(Message::OnInput("2".to_string())),
            key::Key::Character("3") => Some(Message::OnInput("3".to_string())),
            key::Key::Character("4") => Some(Message::OnInput("4".to_string())),
            key::Key::Character("5") => Some(Message::OnInput("5".to_string())),
            key::Key::Character("6") => Some(Message::OnInput("6".to_string())),
            key::Key::Character("7") => Some(Message::OnInput("7".to_string())),
            key::Key::Character("8") => Some(Message::OnInput("8".to_string())),
            key::Key::Character("9") => Some(Message::OnInput("9".to_string())),
            key::Key::Character(",") | key::Key::Character(".") => {
                Some(Message::OnInput(".".to_string()))
            }
            key::Key::Character("+") => Some(Message::Add),
            key::Key::Character("-") => Some(Message::Subtract),
            key::Key::Character("/") => Some(Message::Devide),
            key::Key::Character("*") => Some(Message::Multiply),
            Key::Named(key::Named::Backspace) => Some(Message::OnInput("b".to_string())),
            Key::Named(key::Named::Escape) => Some(Message::Clear),
            Key::Named(key::Named::Enter) => Some(Message::Answer),
            _ => None,
        })
    }

    #[allow(clippy::too_many_lines)]
    fn view(&self) -> iced::Element<'_, Self::Message> {
        let zero = button(
            text("0")
                .size(25)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::FillPortion(2))
        .height(Length::Fill)
        .on_press(Message::OnInput("0".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let comma = button(
            text(",")
                .size(25)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .on_press(Message::OnInput(",".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let equals = button(
            text("=")
                .size(25)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .on_press(Message::Answer)
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Arth)));

        let row_0 = row![zero, comma, equals];

        let one = button(
            text("1")
                .size(25)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::OnInput("1".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let two = button(
            text("2")
                .size(25)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::OnInput("2".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let three = button(
            text("3")
                .size(25)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::OnInput("3".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let add = button(
            text("+")
                .size(25)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::Add)
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Arth)));

        let row_1 = row![one, two, three, add];

        let four = button(
            text("4")
                .size(25)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::OnInput("4".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let five = button(
            text("5")
                .size(25)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::OnInput("5".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let six = button(
            text("6")
                .size(25)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::OnInput("6".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let subtract = button(
            text("-")
                .size(25)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::Subtract)
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Arth)));

        let row_2 = row![four, five, six, subtract];

        let seven = button(
            text("7")
                .size(25)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::OnInput("7".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let eight = button(
            text("8")
                .size(25)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::OnInput("8".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let nine = button(
            text("9")
                .size(25)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::OnInput("9".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let multiply = button(
            text("x")
                .size(25)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::Multiply)
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Arth)));

        let row_3 = row![seven, eight, nine, multiply];

        let clear = button(
            text(if self.lhs.is_empty() {
                "AC".to_string()
            } else {
                "C".to_string()
            })
            .size(20)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::Clear)
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Math)));

        let plus_minus = button(
            text("±")
                .size(25)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::Add)
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Math)));

        let percentage = button(
            text("%")
                .size(25)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::Add)
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Math)));

        let devide = button(
            text("÷")
                .size(25)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::Devide)
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Arth)));

        let row_4 = row![clear, plus_minus, percentage, devide];

        let calculation = container(
            text(
                match (
                    self.lhs.as_str(),
                    self.rhs.as_str(),
                    self.operator,
                    self.result == 0.0,
                ) {
                    ("", "", None, true) => "0".to_owned(),
                    (lhs, "", None | Some(_), true) => lhs.to_owned(),
                    (_, rhs, Some(_), true) => rhs.to_owned(),
                    (_, _, Some(_), false) => self.result.to_string(),
                    _ => "0".to_owned(),
                },
            )
            .size(match self.lhs.len() {
                0..=9 => 40,
                10 => 35,
                11 => 30,
                12 => 25,
                13 => 20,
                _ => 15,
            }),
        )
        .align_x(Horizontal::Right)
        .align_y(Vertical::Bottom)
        .width(Length::Fill)
        .height(80)
        .padding([0, 5])
        .style(theme::Container::Custom(Box::new(
            styling::InputResultContainer(self.theme().palette()),
        )));

        return column![
            calculation,
            row_4.height(Length::FillPortion(1)),
            row_3.height(Length::FillPortion(1)),
            row_2.height(Length::FillPortion(1)),
            row_1.height(Length::FillPortion(1)),
            row_0.height(Length::FillPortion(1))
        ]
        .spacing(0.0)
        .into();
    }
}
