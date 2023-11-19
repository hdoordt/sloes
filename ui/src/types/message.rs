use super::pages::Page;

#[derive(Debug, Clone)]
pub enum Message {
    // Switch from
    SwitchTab(Page), // TODO make types for pages
}
