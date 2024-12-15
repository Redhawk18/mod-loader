use iced::{
    widget::{button, column, pick_list, row},
    Element,
};

use mod_loader_loader::{
    game::Game,
    profile::{list_profiles, Profile},
};

#[derive(Debug, Clone)]
pub enum Message {
    Select,
    SelectProfile(Profile),
    Create,
    Import,
    Update,
    Delete,
}

pub fn view<'a>(current: Option<Profile>) -> Element<'a, Message> {
    let profiles = list_profiles(Game::LethalCompany);
    column![
        row![pick_list(profiles, current, Message::SelectProfile)],
        row![
            button("Select").on_press(Message::Select),
            button("Create").on_press(Message::Create),
            button("Import").on_press(Message::Import),
            button("Update").on_press(Message::Update),
            button("Delete").on_press(Message::Delete),
        ]
    ]
    .into()
}
