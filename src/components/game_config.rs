use anyhow::Result;

#[derive(Debug)]
pub struct GameConfig {
    pub fullscreen: bool,
    pub map_file: String,
}
impl Default for GameConfig {
    fn default() -> Self {
        Self {
            fullscreen: false,
            map_file: "".to_string(),
        }
    }
}

impl GameConfig {
    pub fn new_from_file(&mut self, file_path: &str) -> Result<()> {
        Ok(())
    }
}
