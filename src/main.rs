mod asset_manager;
mod camera;
mod game;
mod game_config;
mod game_state;

use anyhow::Result;
use hexx::{HexLayout, HexOrientation, Vec2};

use crate::{game::Game, game_config::GameConfig};

#[macroquad::main("Hello-Macroquad")]
async fn main() -> Result<()> {
    let mut game = Game::new(GameConfig {
        fullscreen: false,
        hex_layout: HexLayout {
            scale: Vec2::new(48., 48.),
            orientation: HexOrientation::Pointy,
            origin: Vec2::new(0., 0.),
        },
    });
    game.run().await?;
    Ok(())
}
