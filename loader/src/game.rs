use std::fmt::Display;

pub enum Game {
    LethalCompany,
}

impl Game {
    pub fn to_str(&self) -> &str {
        match self {
            Game::LethalCompany => "Lethal Company",
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
