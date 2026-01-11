pub struct Entity {
    pub image: String,
    pub rotation: f32,
    pub data: Option<EntityData>,
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            image: String::new(),
            rotation: 0.,
            data: None,
        }
    }
}

impl Entity {
    pub fn new(image: String) -> Self {
        Self {
            image,
            rotation: 0.,
            data: None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EntityData {}
