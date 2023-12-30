// use crate::styling;
// use iced::alignment::{Horizontal, Vertical};
// use iced::theme::Button;
// use iced::widget::{button, column, container, row, text};
// use iced::Length;

// pub fn button_view(txt: String, message: Message, style: button::Appearance) {
//     let zero = button(
//         text(format!("{txt}"))
//             .size(25)
//             .horizontal_alignment(Horizontal::Center)
//             .vertical_alignment(Vertical::Center),
//     )
//     .width(Length::FillPortion(2))
//     .on_press(Message::OnInput("0".to_string()))
//     .padding([5, 10])
//     .style(theme::Button::Custom(Box::new(style)));
//     zero
// }

// // pub fn initial_page(sniffer: &Sniffer) -> Container<Message, Renderer<StyleType>> {
// //     let style = sniffer.settings.style;
// //     let language = sniffer.settings.language;
// //     let color_gradient = sniffer.settings.color_gradient;
// //     let font = style.get_extension().font;

// //     let col_adapter = get_col_adapter(sniffer, font);

// //     let ip_active = &sniffer.filters.ip_versions;
// //     let col_ip_buttons = col_ip_buttons(ip_active, font, language);

// //     let protocol_active = &sniffer.filters.protocols;
// //     let col_protocol_buttons = col_protocol_buttons(protocol_active, font, language);

// //     let address_active = &sniffer.filters.address_str;
// //     let col_address_filter = col_address_input(address_active, font, language);

// //     let port_active = &sniffer.filters.port_str;
// //     let col_port_filter = col_port_input(port_active, font, language);

// //     let filters_pane = Column::new()
// //         .width(FillPortion(6))
// //         .padding(10)
// //         .spacing(15)
// //         .push(
// //             select_filters_translation(language)
// //                 .font(font)
// //                 .style(TextType::Title)
// //                 .size(FONT_SIZE_TITLE),
// //         )
// //         .push(
// //             Row::new()
// //                 .spacing(20)
// //                 .push(col_ip_buttons)
// //                 .push(col_protocol_buttons),
// //         )
// //         .push(
// //             Row::new()
// //                 .spacing(20)
// //                 .push(col_address_filter)
// //                 .push(col_port_filter),
// //         )
// //         .push(Rule::horizontal(40))
// //         .push(
// //             Container::new(button_start(
// //                 font,
// //                 language,
// //                 color_gradient,
// //                 &sniffer.filters,
// //             ))
// //             .width(Length::Fill)
// //             .height(Length::Fill)
// //             .align_y(Vertical::Center)
// //             .align_x(Horizontal::Center),
// //         );

// //     let body = Column::new().push(vertical_space(Length::Fixed(5.0))).push(
// //         Row::new()
// //             .push(col_adapter)
// //             .push(horizontal_space(Length::Fixed(30.0)))
// //             .push(filters_pane),
// //     );

// //     Container::new(body).height(Length::Fill)
// // }
