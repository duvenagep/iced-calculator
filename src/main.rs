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
use styling::Button::{Arth, Math, Num};
use view::{button_view, buttons_collect};

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

#[derive(Debug, Clone, Default)]
struct Calculator {
    lhs: String,
    rhs: String,
    operator: Option<char>,
    result: f64,
}

#[derive(Clone, Debug)]
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
            Message::Add => {
                self.operator = Some('+');
                self.rhs = "".to_string();
            }
            Message::Subtract => {
                self.operator = Some('-');
                self.rhs = "".to_string();
            }
            Message::Multiply => {
                self.operator = Some('x');
                self.rhs = "".to_string();
            }
            Message::Devide => {
                self.operator = Some('÷');
                self.rhs = "".to_string();
            }
            Message::Clear => {
                *self = Self::default();
            }
            Message::OnInput(val) => {
                if val == "b" && self.operator.is_none() {
                    self.lhs.pop();
                    self.result = self.lhs.replace(",", ".").parse::<f64>().unwrap();
                } else if val == "b" && !self.operator.is_none() {
                    self.rhs.pop();
                    self.result = self.rhs.replace(",", ".").parse::<f64>().unwrap();
                } else if self.operator.is_none() {
                    self.lhs.push_str(&val);
                    self.result = self.lhs.replace(",", ".").parse::<f64>().unwrap();
                } else {
                    self.rhs.push_str(&val);
                    self.result = self.rhs.replace(",", ".").parse::<f64>().unwrap();
                }
            }

            Message::Answer => {
                let l = self.lhs.replace(",", ".").parse::<f64>().unwrap();
                let r = self.rhs.replace(",", ".").parse::<f64>().unwrap();

                self.result = match self.operator {
                    Some('+') => l + r,
                    Some('-') => l - r,
                    Some('x') => l * r,
                    Some('÷') => l / r,
                    Some(_) | None => 0.0,
                };

                self.lhs = self.result.to_string();
            }
        }
        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        keyboard::on_key_press(|key, _modifiers| match key.as_ref() {
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
        // let nums = (1..=9).into_iter().map(|n| n.to_string()).collect();
        // let btns: Vec<Button<'static, Message>> = buttons_collect(nums, Num, 1);

        let zero = button_view("0", Message::OnInput("0".to_string()), 2, Num);
        let comma = button_view(",", Message::OnInput(",".to_string()), 1, Num);
        let equals = button_view("=", Message::Answer, 1, Arth);

        let row_0 = row![zero, comma, equals];

        let one = button_view("1", Message::OnInput("1".to_string()), 1, Num);
        let two = button_view("2", Message::OnInput("2".to_string()), 1, Num);
        let three = button_view("3", Message::OnInput("3".to_string()), 1, Num);
        let add = button_view("+", Message::Add, 1, Arth);

        let row_1 = row![one, two, three, add];

        let four = button_view("4", Message::OnInput("4".to_string()), 1, Num);
        let five = button_view("5", Message::OnInput("5".to_string()), 1, Num);
        let six = button_view("6", Message::OnInput("6".to_string()), 1, Num);
        let subtract = button_view("-", Message::Subtract, 1, Arth);

        let row_2 = row![four, five, six, subtract];

        let seven = button_view("7", Message::OnInput("7".to_string()), 1, Num);
        let eight = button_view("8", Message::OnInput("8".to_string()), 1, Num);
        let nine = button_view("9", Message::OnInput("9".to_string()), 1, Num);
        let multiply = button_view("x", Message::Multiply, 1, Arth);

        let row_3 = row![seven, eight, nine, multiply];

        let clear = button_view(
            if self.lhs.is_empty() {
                "AC".to_string()
            } else {
                "C".to_string()
            },
            Message::Clear,
            1,
            Math,
        );
        let plus_minus = button_view("±", Message::Add, 1, Math);
        let percentage = button_view("%", Message::Add, 1, Math);
        let devide = button_view("÷", Message::Devide, 1, Arth);

        let row_4 = row![clear, plus_minus, percentage, devide];

        let calculation = container(text(self.result.to_string()).size(match self.lhs.len() {
            0..=9 => 40,
            10 => 37,
            11 => 34,
            12 => 31,
            13 => 28,
            14 => 25,
            15 => 22,
            16 => 19,
            17 => 16,
            _ => 16,
        }))
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
