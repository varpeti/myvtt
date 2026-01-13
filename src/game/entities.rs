pub mod default;
pub mod entity;

use std::{collections::HashMap, f32};

use anyhow::Result;
use hexx::{Hex, HexLayout};
use macroquad::prelude::*;

use crate::game::{
    entities::{default::EntityEvent, entity::Entity},
    events::Events,
    map::q2h,
    theme::Theme,
};

pub struct Entities {
    entities: Vec<Entity>,
    grabbed_entity: Option<usize>,
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

    pub fn update(&mut self, dt: f32) -> Result<()> {
        for entity in self.entities.iter_mut() {
            entity.update(dt);
        }
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

            if drag && let Some((eid, entity)) = self.get_mut_entity_by_hex(hex) {
                entity.to_alpha = 0.5;
                self.grabbed_entity = Some(eid);
            }

            if drop && let Some(eid) = self.grabbed_entity {
                let mut entity = self.entities.remove(eid);
                entity.to_alpha = 1.0;
                entity.hex = hex;
                self.entities.push(entity);
                self.grabbed_entity = None;
            }

            if duplicate_drag && let Some((_eid, entity)) = self.get_entity_by_hex(hex) {
                let mut entity = entity.clone();
                entity.to_alpha = 0.5;
                self.entities.push(entity);
                self.grabbed_entity = Some(self.entities.len() - 1);
            }

            if remove {
                self.pop_entity_by_hex(hex);
            }
        }

        // TODO: Smooth
        let size_up = self.entity_events.pop(&EntityEvent::SizeUp);
        let size_down = self.entity_events.pop(&EntityEvent::SizeDown);
        let rotate_clockwise = self.entity_events.pop(&EntityEvent::RotateClockwise);
        let rotate_anticlockwise = self.entity_events.pop(&EntityEvent::RotateAntiClockwise);

        if (size_up || size_down || rotate_clockwise || rotate_anticlockwise)
            && let Some(eid) = self.grabbed_entity
            && let Some(entity) = self.entities.get_mut(eid)
        {
            if size_up {
                entity.to_size *= self.size_gamma;
            }
            if size_down {
                entity.to_size *= 1. / self.size_gamma;
            }
            if rotate_clockwise {
                entity.to_rotation += self.rotation_delta;
            }
            if rotate_anticlockwise {
                entity.to_rotation -= self.rotation_delta;
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
        let hex_inradius_size = hex_layout.scale.x * Self::INRADIUS_2;
        for entity in self.entities.iter() {
            entity.draw(
                hex_layout,
                hex_inradius_size,
                &self.textures,
                self.entity_esp,
                theme,
            );
        }

        if let Some(id) = self.grabbed_entity
            && let Some(entity) = self.entities.get(id)
        {
            let pos = camera.screen_to_world(mouse_position().into());
            entity.draw_to(
                pos,
                hex_inradius_size,
                &self.textures,
                self.entity_esp,
                theme,
            );
        }
    }

    pub fn get_mut_entity_by_hex(&mut self, hex: Hex) -> Option<(usize, &mut Entity)> {
        self.entities
            .iter_mut()
            .enumerate()
            .rfind(|(_, e)| e.hex == hex)
    }

    pub fn get_entity_by_hex(&mut self, hex: Hex) -> Option<(usize, &Entity)> {
        self.entities
            .iter()
            .enumerate()
            .rfind(|(_, e)| e.hex == hex)
    }

    pub fn pop_entity_by_hex(&mut self, hex: Hex) -> Option<Entity> {
        if let Some((id, _)) = self
            .entities
            .iter()
            .enumerate()
            .rfind(|(_, e)| e.hex == hex)
        {
            Some(self.entities.remove(id))
        } else {
            None
        }
    }

    pub const INRADIUS: f32 = 0.866_025_4; // sqrt(3)/2
    pub const INRADIUS_2: f32 = 1.732_050_8; // sqrt(3)/2 * 2
}
