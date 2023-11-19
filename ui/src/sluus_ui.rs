use iced::executor;
use iced::{Application, Command, Element, Theme};

use crate::components::header;
use crate::types::message::Message;
use crate::types::pages::Page;

pub struct SluusUi {
    active_page: Page,
}

impl Default for SluusUi {
    fn default() -> Self {
        Self {
            active_page: Page::Proxy,
        }
    }
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

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SwitchTab(tab) => self.active_page = tab,
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        // header -> proxy, brute, replay, scan?, discover?
        // body -> depends on tab selected in header
        //
        // ...
        header(&self.active_page).into()
    }
}
