use iced::{widget::button, Element};

#[derive(Debug, Clone)]
pub enum Message {
    LethalCompany,
}

pub fn view<'a>() -> Element<'a, Message> {
    button("Lethal Company")
        .on_press(Message::LethalCompany)
        .into()
}
