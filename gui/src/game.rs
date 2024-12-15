use std::fmt::Display;

pub enum Game {
    LethalCompany,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Game::LethalCompany => "Lethal Company",
            }
        )
    }
}
