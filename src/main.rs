mod components;
mod game;

use anyhow::Result;
use three_d::*;

pub fn main() -> Result<()> {
    let window = Window::new(WindowSettings {
        title: "myvtt".to_string(),
        borderless: true,
        ..Default::default()
    })?;

    let mut game = game::Game::default();

    game.load(&window.gl())?;

    window.render_loop(move |mut frame_input| {
        game.run(&mut frame_input)
            .unwrap_or_else(|err| panic!("{}", err))
    });
    Ok(())
}
