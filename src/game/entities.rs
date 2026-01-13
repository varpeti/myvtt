pub mod default;
pub mod entity;

use std::{
    collections::{HashMap, HashSet},
    f32,
};

use anyhow::Result;
use hexx::{Hex, HexLayout};
use macroquad::prelude::*;

use crate::game::{
    entities::{default::EntityEvent, entity::Entity},
    events::Events,
    map::{h2q, q2h},
    theme::{Theme, ThemeColor},
};

pub struct Entities {
    entities: HashMap<Hex, Vec<Entity>>,
    selected_entities: HashSet<Hex>,
    grabbed_entities: HashSet<Hex>,
    textures: HashMap<String, Texture2D>,
    entity_events: Events<EntityEvent>,

    entity_esp: bool,

    rotation_delta: f32,
    size_gamma: f32,
}

impl Entities {
    pub async fn load_textures(&mut self) -> Result<()> {
        // TODO: load all files
        self.load_texture("Token_Template.png").await?;
        self.load_texture("dragon, amethyst.png").await?;
        self.load_texture("bandit001.png").await?;
        self.load_texture("bandit002.png").await?;
        self.load_texture("red_dragon.png").await?;
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

        let drag = self.entity_events.pop(&EntityEvent::Drag);
        let drop = self.entity_events.pop(&EntityEvent::Drop);
        let duplicate_drag = self.entity_events.pop(&EntityEvent::DuplicateDrag);
        let remove = self.entity_events.pop(&EntityEvent::Remove);

        if drag || drop || duplicate_drag || remove {
            let hex =
                hex_layout.world_pos_to_hex(q2h(camera.screen_to_world(mouse_position().into())));

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

            if duplicate_drag
                && let Some(entities) = self.entities.get_mut(&hex)
                && let Some(entity) = entities.last()
            {
                let mut entity = entity.clone();
                entity.alpha = 0.5;
                self.grabbed_entities.insert(hex);
                entities.push(entity);
            }

            if remove && let Some(mut entities) = self.entities.remove(&hex) {
                entities.pop();
                if !entities.is_empty() {
                    self.entities.insert(hex, entities);
                }
            }
        }

        // TODO: Smooth
        let size_up = self.entity_events.pop(&EntityEvent::SizeUp);
        let size_down = self.entity_events.pop(&EntityEvent::SizeDown);
        let rotate_clockwise = self.entity_events.pop(&EntityEvent::RotateClockwise);
        let rotate_anticlockwise = self.entity_events.pop(&EntityEvent::RotateAntiClockwise);

        if size_up || size_down || rotate_clockwise || rotate_anticlockwise {
            for hex in self.grabbed_entities.iter() {
                if let Some(entities) = self.entities.get_mut(hex)
                    && let Some(entity) = entities.last_mut()
                {
                    if size_up {
                        entity.size *= self.size_gamma;
                    }
                    if size_down {
                        entity.size *= 1. / self.size_gamma;
                    }
                    if rotate_clockwise {
                        entity.rotation += self.rotation_delta;
                    }
                    if rotate_anticlockwise {
                        entity.rotation -= self.rotation_delta;
                    }
                }
            }
        }

        if self.entity_events.pop(&EntityEvent::ToggleEntityEsp) {
            self.entity_esp = !self.entity_esp;
        }

        // Only works in WASM32...
        for file in get_dropped_files() {
            info!("{:?}", file.path);
        }

        // Only works in WASM32...
        for file_id in 0..miniquad::window::dropped_file_count() {
            info!("minquad {:?}", miniquad::window::dropped_file_path(file_id));
        }

        Ok(())
    }

    pub fn draw(&self, theme: &Theme, hex_layout: &HexLayout, camera: &Camera2D) {
        let hex_inradius_size = h2q(hex_layout.scale) * Self::INRADIUS_2;
        for (&hex, entities) in self.entities.iter() {
            let pos = h2q(hex_layout.hex_to_world_pos(hex));
            for entity in entities.iter() {
                entity.draw(pos, hex_inradius_size, &self.textures);
            }
            if self.entity_esp {
                draw_circle_lines(
                    pos.x,
                    pos.y,
                    hex_layout.scale.x,
                    entities.len() as f32,
                    theme.color(ThemeColor::Normal),
                );
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
                entity.draw(pos, hex_inradius_size, &self.textures);
            }
        }
    }

    pub const INRADIUS: f32 = 0.866_025_4; // sqrt(3)/2
    pub const INRADIUS_2: f32 = 1.732_050_8; // sqrt(3)/2 * 2
}
