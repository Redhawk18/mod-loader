use std::{fmt::Display, fs::create_dir_all, io, path::PathBuf};

use dirs::config_dir;
use tracing::debug;

use crate::{game::Game, BINARY_NAME};

#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    name: String,
    path: PathBuf,
}

impl Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn create_profile(profile: Profile) -> io::Result<()> {
    todo!()
}

pub fn delete_profile(profile: Profile) -> io::Result<()> {
    todo!()
}

pub fn list_profiles(game: Game) -> Vec<Profile> {
    let path = config_dir()
        .expect("Unsupported platfrom by `dirs` crate")
        .join(BINARY_NAME.to_owned() + "/" + game.to_str() + "/profiles");

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
