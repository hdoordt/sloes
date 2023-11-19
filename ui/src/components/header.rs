use enum_iterator::all;
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, Container, Row, Text},
    Alignment, Length, Renderer,
};

use crate::types::{message::Message, pages::Page};

pub fn header(active_page: &Page) -> Container<'static, Message, Renderer> {
    Container::new(header_tabs(active_page)) // TODO pass in state as page type
        .align_y(Vertical::Top)
        .width(Length::Fill)
        .height(Length::Fixed(30.0))
}

pub fn header_tabs(active: &Page) -> Row<'static, Message, Renderer> {
    let mut res = Row::new()
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(10);
    for page in all::<Page>() {
        // TODO if active == tab
        // let title: Element<Message> = page.into();
        let mut btn = button(Text::new(page.to_string()).horizontal_alignment(Horizontal::Center))
            .height(Length::Fill)
            .width(Length::FillPortion(1));
        if page != *active {
            btn = btn.on_press(Message::SwitchTab(page));
        }
        res = res.push(btn)
    }
    res
}
