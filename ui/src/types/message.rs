use iced::widget::scrollable;

use super::pages::Page;

#[derive(Debug, Clone)]
pub enum Message {
    SwitchTab(Page),
    SelectRequest(u8), // TODO make this a type?
    ScrollProxyRequests(scrollable::RelativeOffset),
}
