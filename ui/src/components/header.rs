use enum_iterator::all;
use iced::{
    alignment::Horizontal,
    widget::{button, Row, Text},
    Alignment, Length, Renderer,
};

use crate::types::{message::Message, pages::Page};

pub fn header(active_page: &Page) -> Row<'static, Message, Renderer> {
    let mut tabs = Row::new()
        .height(Length::Fixed(30.0))
        .width(Length::Fill)
        // .align_items(Alignment::Center)
        .spacing(10);
    for page in all::<Page>() {
        // TODO if active == tab
        // let title: Element<Message> = page.into();
        let mut btn = button(Text::new(page.to_string()).horizontal_alignment(Horizontal::Center))
            .height(Length::Fill)
            .width(Length::FillPortion(1));
        if page != *active_page {
            btn = btn.on_press(Message::SwitchTab(page));
        }
        tabs = tabs.push(btn)
    }
    tabs
}
