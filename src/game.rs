use anyhow::Result;
use three_d::*;

use crate::components::{camera::RPGCamera, game_config::GameConfig, map::Map};

#[derive(Default)]
#[allow(dead_code)]
pub struct Game {
    config: GameConfig,
    camera: RPGCamera,
    map: Map,
}

#[allow(dead_code)]
impl Game {
    pub fn load(&mut self, context: &Context) -> Result<()> {
        self.map.load_models(context)?;
        Ok(())
    }

    pub fn run(&mut self, frame_input: &mut FrameInput) -> Result<FrameOutput> {
        self.handle_events(frame_input)?;
        self.draw(frame_input)?;
        Ok(FrameOutput::default())
    }

    fn handle_events(&mut self, frame_input: &mut FrameInput) -> Result<()> {
        self.camera.handle_events(frame_input)?;
        self.map.handle_events(frame_input)?;
        Ok(())
    }

    fn draw(&mut self, frame_input: &mut FrameInput) -> Result<()> {
        frame_input.screen().clear(ClearState::color_and_depth(
            0.09804, 0.0902, 0.14118, 1., 1.,
        ));
        let camera = self.camera.get_camera();
        self.map.draw(frame_input, camera)?;
        Ok(())
    }
}
