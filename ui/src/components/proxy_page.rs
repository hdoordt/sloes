use iced::{
    application::StyleSheet,
    widget::{button, scrollable, Column, Container, Row, Scrollable, Text},
    Font, Length, Renderer,
};
use lazy_static::lazy_static;

use crate::{sluus_ui::ProxyState, types::message::Message};

lazy_static! {
    pub static ref PROXY_REQUESTS_SCROLLABLE_ID: scrollable::Id = scrollable::Id::unique();
}

pub fn proxy_page(state: &ProxyState) -> Row<'static, Message, Renderer> {
    let mut page = Column::new();

    let mut request_table = Column::new();
    // TODO fill with requests
    for i in (0..20).rev() {
        let mut request = button(
            Row::new()
                .push(Text::new(i.to_string()).width(Length::FillPortion(1)))
                .push(Text::new("https://domain.nl".to_string()).width(Length::FillPortion(5)))
                .push(Text::new("MTHD".to_string()).width(Length::FillPortion(2)))
                .push(Text::new("200".to_string()).width(Length::FillPortion(2)))
                .width(Length::FillPortion(1)),
        )
        .width(Length::Fill)
        .height(Length::Shrink);

        if !state.selected_request.is_some_and(|x| x == i) {
            request = request.on_press(Message::SelectRequest(i));
        }

        request_table = request_table.push(request);
    }

    let scroll = Scrollable::new(request_table)
        .on_scroll(|v| Message::ScrollProxyRequests(v.relative_offset()))
        .id(PROXY_REQUESTS_SCROLLABLE_ID.clone());
    page = page.push(Row::new().height(Length::FillPortion(1)).push(scroll));
    // TODO use request
    if let Some(x) = state.selected_request {
        let request = Column::new()
            .padding(10)
            .width(Length::FillPortion(1))
            .push(Text::new("Request"))
            .push(Container::new(
                Text::new("Hello\n123").font(Font::MONOSPACE),
            ));
        let response = Column::new()
            .padding(10)
            .width(Length::FillPortion(1))
            .push(Text::new("Response"))
            .push(Container::new(
                Text::new("Hello\n123").font(Font::MONOSPACE),
            ));

        page = page.push(
            Row::new()
                .height(Length::FillPortion(1))
                .push(request)
                .push(response),
        );
    }

    Row::new().push(page)
}
