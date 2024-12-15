use std::{fmt::Display, fs::create_dir_all, path::PathBuf};

use dirs::config_dir;
use iced::{
    widget::{button, column, pick_list, row},
    Element,
};
use tracing::debug;

use crate::BINARY_NAME;

#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    name: String,
    path: PathBuf,
}

#[derive(Debug, Clone)]
pub enum Message {
    Select,
    SelectProfile(Profile),
    Create,
    Import,
    Update,
    Delete,
}

impl Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn view<'a>(current: Option<Profile>) -> Element<'a, Message> {
    let profiles = list_profiles();
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

fn list_profiles() -> Vec<Profile> {
    let path = config_dir()
        .expect("Unsupported platfrom by `dirs` crate")
        .join(BINARY_NAME.to_owned() + "/LethalCompany" + "/profiles");

    debug!("Looking for config path at {:?}", path);

    if !path.exists() || !path.is_dir() {
        if let Err(error) = create_dir_all(path.clone()) {
            panic!("error {}", error);
        }
    }
    debug!("Found or created config path");

    let mut profiles: Vec<Profile> = vec![];

    let root = std::fs::read_dir(path).unwrap();
    for folder in root {
        println!("{}", folder.as_ref().unwrap().path().display());
        let name = folder.as_ref().unwrap().file_name().into_string().unwrap();
        let path = folder.as_ref().unwrap().path();
        profiles.push(Profile { name, path });
    }

    profiles
}
