pub mod game;

use anyhow::Result;

use crate::game::Game;

#[macroquad::main("myvtt")]
async fn main() -> Result<()> {
    let mut game = Game::default();
    game.load().await?;
    game.run().await?;
    Ok(())
}
