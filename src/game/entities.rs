pub mod entity;

use std::{
    collections::{HashMap, HashSet},
    f32,
};

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
    entities: HashMap<Hex, Vec<Entity>>,
    selected_entities: HashSet<Hex>,
    grabbed_entities: HashSet<Hex>,
    textures: HashMap<String, Texture2D>,
    entity_events: Events<EntityEvent>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum EntityEvent {
    Select,
    Drag,
    Drop,
}

impl Event for EntityEvent {}

impl Default for Entities {
    fn default() -> Self {
        // TODO: Remove
        let todo_remove = HashMap::from([
            (
                Hex::new(0, 0),
                vec![Entity::new("FullTransparentGreen".to_string())],
            ),
            (
                Hex::new(0, 1),
                vec![Entity::new("Token_Template.png".to_string())],
            ),
            (
                Hex::new(0, 2),
                vec![Entity::new("dragon, amethyst.png".to_string())],
            ),
        ]);
        Self {
            entities: todo_remove, //HashMap::new(),
            selected_entities: HashSet::new(),
            grabbed_entities: HashSet::new(),
            textures: HashMap::from([(
                "FullTransparentGreen".to_string(),
                Texture2D::from_image(&Image::gen_image_color(8, 8, GREEN.with_alpha(0.75))),
            )]),
            entity_events: Events::from([
                (
                    EntityEvent::Select,
                    vec![vec![(
                        EventS::JustPressed,
                        EventT::Mouse(MouseButton2::LeftClick),
                    )]],
                ),
                (
                    EntityEvent::Drag,
                    vec![vec![(
                        EventS::JustPressed,
                        EventT::Mouse(MouseButton2::LeftClick),
                    )]],
                ),
                (
                    EntityEvent::Drop,
                    vec![vec![(
                        EventS::JustReleased,
                        EventT::Mouse(MouseButton2::LeftClick),
                    )]],
                ),
            ]),
        }
    }
}

impl Entities {
    pub async fn load_textures(&mut self) -> Result<()> {
        Ok(())
    }

    pub async fn load_texture(&mut self, name: &str) -> Result<()> {
        let path = &format!("assets/img/{}", name);
        let image = load_image(path).await?;
        let texture = Texture2D::from_image(&image);
        self.textures.insert(name.to_string(), texture);
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

        let select = self.entity_events.pop(&EntityEvent::Select);
        let drag = self.entity_events.pop(&EntityEvent::Drag);
        let drop = self.entity_events.pop(&EntityEvent::Drop);

        if select || drag || drop {
            let hex =
                hex_layout.world_pos_to_hex(q2h(camera.screen_to_world(mouse_position().into())));

            // if select && self.entities.contains_key(&hex) && !self.selected_entities.remove(&hex) {
            //     self.selected_entities.insert(hex);
            // }

            if drag
                && let Some(entities) = self.entities.get_mut(&hex)
                && let Some(entity) = entities.last_mut()
            {
                entity.alpha = 0.5;
                self.grabbed_entities.insert(hex);
            }

            if drop {
                for old_hex in self.grabbed_entities.iter() {
                    if old_hex == hex {
                        if let Some(entities) = self.entities.get_mut(old_hex)
                            && let Some(entity) = entities.last_mut()
                        {
                            entity.alpha = 1.0;
                        }
                    } else if let Some(mut entities) = self.entities.remove(old_hex)
                        && let Some(mut entity) = entities.pop()
                    {
                        entity.alpha = 1.0;
                        self.entities.entry(hex).or_default().push(entity);
                        if !entities.is_empty() {
                            self.entities.insert(*old_hex, entities);
                        }
                    }
                }
                self.grabbed_entities.clear();
            }
        }

        Ok(())
    }

    pub fn draw(&self, theme: &Theme, hex_layout: &HexLayout, camera: &Camera2D) {
        let size = h2q(hex_layout.scale) * Self::INRADIUS_2;
        for (&hex, entities) in self.entities.iter() {
            for entity in entities.iter() {
                let pos = h2q(hex_layout.hex_to_world_pos(hex));
                entity.draw(pos, size, &self.textures);
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

        for hex in self.grabbed_entities.iter() {
            if let Some(entities) = self.entities.get(hex)
                && let Some(entity) = entities.last()
            {
                let pos = camera.screen_to_world(mouse_position().into());
                entity.draw(pos, size, &self.textures);
            }
        }
    }

    pub const INRADIUS_2: f32 = 1.732_050_8; // (inradius = sqrt(3)/2) * 2
}
