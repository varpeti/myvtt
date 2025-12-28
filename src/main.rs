mod components;
mod game;
mod game_config;
mod game_state;

use anyhow::Result;

use crate::{game::Game, game_config::GameConfig};

#[macroquad::main("Hello-Macroquad")]
async fn main() -> Result<()> {
    let mut game = Game::new(GameConfig { fullscreen: false });
    game.run().await?;
    Ok(())
}
