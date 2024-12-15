pub mod screens;

use iced::{Element, Task};
use screens::{
    profile::{self},
    selection::{self},
};
use tracing::debug;

use mod_loader_loader::{game::Game, profile::Profile};

pub fn run() -> iced::Result {
    iced::application("mod-loader", App::update, App::view).run_with(App::new)
}

pub struct App {
    game: Option<Game>,
    profile: Option<Profile>,
    screen: Screen,
}

#[derive(Default)]
pub enum Screen {
    Profile,
    #[default]
    Selection,
}

#[derive(Debug, Clone)]
pub enum Message {
    Profile(profile::Message),
    Selection(selection::Message),
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (
            App {
                game: None,
                profile: None,
                screen: Screen::default(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        let game;

        match message {
            Message::Selection(msg) => {
                match msg {
                    selection::Message::LethalCompany => {
                        game = Game::LethalCompany;
                    }
                }
                debug!("Selected game {}", game);

                self.game = Some(game);
                self.screen = Screen::Profile;
            }
            Message::Profile(msg) => match msg {
                profile::Message::Select => todo!(),
                profile::Message::Create => todo!(),
                profile::Message::Import => todo!(),
                profile::Message::Update => todo!(),
                profile::Message::Delete => todo!(),
                profile::Message::SelectProfile(profile) => self.profile = Some(profile),
            },
        }

        Task::none()
    }

    fn view(&self) -> Element<Message> {
        match self.screen {
            Screen::Selection => selection::view().map(Message::Selection),
            Screen::Profile => profile::view(self.profile.clone()).map(Message::Profile),
        }
    }
}
