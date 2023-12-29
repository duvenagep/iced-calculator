#![warn(clippy::all, clippy::pedantic, clippy::restriction)]

mod styling;

use iced::alignment;
use iced::executor;
use iced::theme;
use iced::widget::{button, column, container, row, text};
use iced::window::PlatformSpecific;
use iced::window::Position;
use iced::{window, Application, Command, Length, Settings, Theme};

fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (232, 321),
            resizable: true,
            decorations: true,
            position: Position::Default,
            min_size: Some((232, 321)),
            max_size: Some((232, 321)),
            visible: true,
            transparent: true,
            platform_specific: PlatformSpecific {
                title_hidden: true,
                titlebar_transparent: true,
                fullsize_content_view: true,
            },
            ..Default::default()
        },
        antialiasing: true,

        ..Default::default()
    };

    Calculator::run(settings)
}

struct Calculator {
    lhs: String,
    rhs: String,
    operator: Option<char>,
    result: usize,
}

#[derive(Debug, Clone)]
enum Message {
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
        (
            Self {
                lhs: "".to_string(),
                rhs: "".to_string(),
                operator: None,
                result: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::OnInput(val) => {
                if self.operator.is_none() {
                    self.lhs.push_str(&val)
                } else {
                    self.rhs.push_str(&val)
                }
            }
            Message::Add => self.operator = Some('+'),
            Message::Subtract => self.operator = Some('-'),
            Message::Multiply => self.operator = Some('x'),
            Message::Devide => self.operator = Some('÷'),
            Message::Clear => {
                self.result = 0;
                self.lhs = "".to_string();
                self.rhs = "".to_string();
                self.operator = None;
            }
            Message::Answer => match self.operator.unwrap() {
                '+' => {
                    self.result =
                        self.lhs.parse::<usize>().unwrap() + self.rhs.parse::<usize>().unwrap()
                }
                '-' => {
                    self.result =
                        self.lhs.parse::<usize>().unwrap() - self.rhs.parse::<usize>().unwrap()
                }
                '×' => {
                    self.result =
                        self.lhs.parse::<usize>().unwrap() * self.rhs.parse::<usize>().unwrap()
                }
                '÷' => {
                    self.result =
                        self.lhs.parse::<usize>().unwrap() / self.rhs.parse::<usize>().unwrap()
                }
                _ => {}
            },
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let zero = button(
            text(format!("0"))
                .size(25)
                .horizontal_alignment(alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .width(Length::FillPortion(2))
        .on_press(Message::OnInput("0".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let comma = button(
            text(format!(","))
                .size(25)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .width(Length::FillPortion(1))
        .on_press(Message::OnInput(",".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let equals = button(
            text(format!("="))
                .size(25)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .width(Length::FillPortion(1))
        .on_press(Message::Answer)
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Arth)));

        let row_0 = row![zero, comma, equals];

        let one = button(
            text(format!("1"))
                .size(25)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .on_press(Message::OnInput("1".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let two = button(
            text(format!("2"))
                .size(25)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .on_press(Message::OnInput("2".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let three = button(
            text(format!("3"))
                .size(25)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .on_press(Message::OnInput("3".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let add = button(
            text(format!("+"))
                .size(25)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .on_press(Message::Add)
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Arth)));

        let row_1 = row![one, two, three, add];

        let four = button(
            text(format!("4"))
                .size(25)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .on_press(Message::OnInput("4".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let five = button(
            text(format!("5"))
                .size(25)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .on_press(Message::OnInput("5".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let six = button(
            text(format!("6"))
                .size(25)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .on_press(Message::OnInput("6".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let subtract = button(
            text(format!("-"))
                .size(25)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .on_press(Message::Subtract)
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Arth)));

        let row_2 = row![four, five, six, subtract];

        let seven = button(
            text(format!("7"))
                .size(25)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .on_press(Message::OnInput("7".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let eight = button(
            text(format!("8"))
                .size(25)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .on_press(Message::OnInput("8".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let nine = button(
            text(format!("9"))
                .size(25)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .on_press(Message::OnInput("9".to_string()))
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Num)));

        let multiply = button(
            text(format!("x"))
                .size(25)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .on_press(Message::Multiply)
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Arth)));

        let row_3 = row![seven, eight, nine, multiply];

        let clear = button(
            text(format!("AC"))
                .size(20)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .on_press(Message::Clear)
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Math)));

        let plus_minus = button(
            text(format!("±"))
                .size(25)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .on_press(Message::Add)
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Math)));

        let percentage = button(
            text(format!("%"))
                .size(25)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .on_press(Message::Add)
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Math)));

        let devide = button(
            text(format!("÷"))
                .size(25)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .on_press(Message::Devide)
        .padding([5, 10])
        .style(theme::Button::Custom(Box::new(styling::Button::Arth)));

        let row_4 = row![clear, plus_minus, percentage, devide];

        let calculation = container(text(&self.result.to_string()).size(40))
            .align_x(alignment::Horizontal::Right)
            .align_y(alignment::Vertical::Bottom)
            .width(Length::Fill)
            .height(80)
            .padding([0, 5])
            .style(theme::Container::Custom(Box::new(
                styling::InputResultContainer(self.theme().palette()),
            )));

        column![
            calculation,
            row_4.height(Length::FillPortion(1)),
            row_3.height(Length::FillPortion(1)),
            row_2.height(Length::FillPortion(1)),
            row_1.height(Length::FillPortion(1)),
            row_0.height(Length::FillPortion(1))
        ]
        .into()
    }
}
