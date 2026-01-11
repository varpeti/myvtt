use anyhow::Result;
use macroquad::prelude::*;

use crate::game::events::{Event, EventS, EventT, Events};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Mode {
    //Menu(),
    Normal,
    MapEditor,
    Exiting,
}

pub struct GameState {
    pub mode: Mode,
    pub fullscreen: bool,
    pub game_events: Events<GameEvent>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum GameEvent {
    ToggleFullScreen,
    SwitchTo(Mode),
}

impl Event for GameEvent {}

#[allow(clippy::derivable_impls)]
impl Default for GameState {
    fn default() -> Self {
        prevent_quit();
        Self {
            mode: Mode::Normal,
            fullscreen: false,
            game_events: Events::from([
                (
                    GameEvent::ToggleFullScreen,
                    vec![
                        vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::F))],
                        vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::F11))],
                    ],
                ),
                (
                    GameEvent::SwitchTo(Mode::Normal),
                    vec![
                        vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Escape))],
                        vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Tab))],
                    ],
                ),
                (
                    GameEvent::SwitchTo(Mode::MapEditor),
                    vec![vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::M))]],
                ),
            ]),
        }
    }
}

impl GameState {
    pub fn handle_events(&mut self, _dt: f32) -> Result<()> {
        self.game_events.update();

        if self.game_events.pop(&GameEvent::ToggleFullScreen) {
            self.fullscreen = !self.fullscreen;
            set_fullscreen(self.fullscreen);
        }

        if self.game_events.pop(&GameEvent::SwitchTo(Mode::Normal)) {
            self.mode = Mode::Normal;
        }

        if self.game_events.pop(&GameEvent::SwitchTo(Mode::MapEditor)) {
            self.mode = Mode::MapEditor;
        }

        Ok(())
    }
}
