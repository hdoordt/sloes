use iced::widget::scrollable::RelativeOffset;
use iced::widget::{scrollable, Column, Container, Row};
use iced::{executor, Length};
use iced::{Application, Command, Element, Theme};

use crate::components::{
    header::header,
    proxy_page::{proxy_page, PROXY_REQUESTS_SCROLLABLE_ID},
};
use crate::types::message::Message;
use crate::types::pages::Page;

// TODO refactor this state stuff
#[derive(Default)]
pub struct ProxyState {
    pub selected_request: Option<u8>, // TODO this should probably be a type
    pub current_scroll_offset: scrollable::RelativeOffset,
}

#[derive(Default)]
pub struct SluusUi {
    active_page: Page,
    proxy_state: ProxyState,
}

impl Application for SluusUi {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Sluus - you decide to drop or forward")
    }

    // TODO refactor message naming
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SwitchTab(tab) => {
                self.active_page = tab;
                return scrollable::snap_to(
                    PROXY_REQUESTS_SCROLLABLE_ID.clone(),
                    self.proxy_state.current_scroll_offset,
                );
            }
            Message::SelectRequest(req) => self.proxy_state.selected_request = Some(req),
            Message::ScrollProxyRequests(offset) => self.proxy_state.current_scroll_offset = offset,
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let header = header(&self.active_page);
        let mut body = Row::new();
        body = match self.active_page {
            Page::Proxy => proxy_page(&self.proxy_state),
            Page::Brute => body,
            Page::Replay => body,
            Page::Scan => body,
            Page::Discover => body,
        };

        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(header)
            .push(body)
            .into()
    }
}
