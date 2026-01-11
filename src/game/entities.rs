pub mod entity;

use std::collections::{HashMap, HashSet};

use anyhow::Result;
use hexx::{Hex, HexLayout};
use macroquad::prelude::*;

use crate::game::{
    entities::entity::Entity,
    events::{Event, EventS, EventT, Events, MouseButton2},
    map::{h2q, q2h},
    theme::{Theme, ThemeColor},
};

pub struct Entities {
    entities: HashMap<Hex, Entity>,
    selected_entities: HashSet<Hex>,
    textures: HashMap<String, Texture2D>,
    entity_events: Events<EntityEvent>,
}

impl Default for Entities {
    fn default() -> Self {
        // TODO: Remove
        let todo_remove = HashMap::from([
            (
                Hex::new(0, 0),
                Entity::new("FullTransparentGreen".to_string()),
            ),
            (Hex::new(0, 1), Entity::new("Token_Template".to_string())),
        ]);
        Self {
            entities: todo_remove, //HashMap::new(),
            selected_entities: HashSet::new(),
            textures: HashMap::from([(
                "FullTransparentGreen".to_string(),
                Texture2D::from_image(&Image::gen_image_color(8, 8, GREEN.with_alpha(0.75))),
            )]),
            entity_events: Events::from([(
                EntityEvent::Select,
                vec![vec![(
                    EventS::JustPressed,
                    EventT::Mouse(MouseButton2::LeftClick),
                )]],
            )]),
        }
    }
}

impl Entities {
    pub async fn load_images(&mut self) -> Result<()> {
        let image = load_image("assets/img/Token_Template.png").await?;
        let texture = Texture2D::from_image(&image);
        self.textures.insert("Token_Template".to_string(), texture);
        Ok(())
    }

    pub async fn load_entities(&mut self) -> Result<()> {
        // TODO:
        Ok(())
    }

    pub fn handle_events(
        &mut self,
        hex_layout: &HexLayout,
        camera: &Camera2D,
        _dt: f32,
    ) -> Result<()> {
        self.entity_events.update();

        if self.entity_events.pop(&EntityEvent::Select) {
            let hex =
                hex_layout.world_pos_to_hex(q2h(camera.screen_to_world(mouse_position().into())));

            if self.entities.contains_key(&hex) && !self.selected_entities.remove(&hex) {
                self.selected_entities.insert(hex);
            }
        }

        Ok(())
    }

    pub fn draw(&self, hex_layout: &HexLayout, theme: &Theme) {
        let hex_rect = hex_layout.rect_size();
        for (&hex, entity) in self.entities.iter() {
            let pos = hex_layout.hex_to_world_pos(hex);
            if let Some(texture) = self.textures.get(&entity.image) {
                draw_texture_ex(
                    texture,
                    pos.x - hex_rect.x / 2.,
                    pos.y - hex_rect.y / 2.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(h2q(hex_rect)),
                        rotation: entity.rotation,
                        ..Default::default()
                    },
                );
            } else {
                draw_circle(pos.x, pos.y, 16., PINK.with_alpha(0.75));
            }
        }

        for hex in self.selected_entities.iter() {
            let pos = hex_layout.hex_to_world_pos(*hex);
            draw_circle_lines(
                pos.x,
                pos.y,
                hex_layout.scale.x,
                3.,
                theme.color(ThemeColor::Normal),
            );
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum EntityEvent {
    Select,
}

impl Event for EntityEvent {}
