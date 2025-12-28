use hexx::Hex;
#[derive(Debug, Default)]
pub struct GameState {
    pub should_exit: bool,
    pub hovered_hex: Hex,
}

impl GameState {
    pub fn new() -> Self {
        let origin = Hex::new(0, 0);
        Self {
            should_exit: false,
            hovered_hex: origin,
        }
    }
}
