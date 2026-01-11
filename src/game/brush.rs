use anyhow::Result;
use macroquad::prelude::*;

use crate::game::{
    events::{Event, EventS, EventT, Events, MouseButton2},
    map::{
        Map, h2q, q2h,
        tiles::{Tile, TileType},
    },
    theme::{Theme, ThemeColor},
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum BrushEvent {
    PickEmpty,
    PickSmall,
    PickHalf,
    PickLarge,
    PickFull,
    RotateClockwise,
    RotateAntiClockwise,
    CloneTile,
    Insert,
    Remove,
    SizeUp,
    SizeDown,
    InsertWalls,
}

impl Event for BrushEvent {}

pub struct Brush {
    brush_pos: Vec2,
    smoothing_factor: f32,
    brush: Tile,
    brush_size: u32,
    brush_events: Events<BrushEvent>,
}

impl Default for Brush {
    fn default() -> Self {
        Self {
            brush_pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
            smoothing_factor: 16.,
            brush: Tile::new(TileType::Empty, 0),
            brush_size: 0,
            brush_events: Events::from([
                (
                    BrushEvent::PickEmpty,
                    vec![vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Key1))]],
                ),
                (
                    BrushEvent::PickSmall,
                    vec![vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Key2))]],
                ),
                (
                    BrushEvent::PickHalf,
                    vec![vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Key3))]],
                ),
                (
                    BrushEvent::PickLarge,
                    vec![vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Key4))]],
                ),
                (
                    BrushEvent::PickFull,
                    vec![vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Key5))]],
                ),
                (
                    BrushEvent::RotateClockwise,
                    vec![
                        vec![(EventS::JustPressed, EventT::Mouse(MouseButton2::WheelUp))],
                        vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Q))],
                    ],
                ),
                (
                    BrushEvent::RotateAntiClockwise,
                    vec![
                        vec![(EventS::JustPressed, EventT::Mouse(MouseButton2::WheelDown))],
                        vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::E))],
                    ],
                ),
                (
                    BrushEvent::SizeUp,
                    vec![
                        vec![(EventS::JustPressed, EventT::Mouse(MouseButton2::WheelUp))],
                        vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Q))],
                    ],
                ),
                (
                    BrushEvent::SizeDown,
                    vec![
                        vec![(EventS::JustPressed, EventT::Mouse(MouseButton2::WheelDown))],
                        vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::E))],
                    ],
                ),
                (
                    BrushEvent::CloneTile,
                    vec![vec![(
                        EventS::JustPressed,
                        EventT::Mouse(MouseButton2::MiddleClick),
                    )]],
                ),
                (
                    BrushEvent::Insert,
                    vec![vec![(
                        EventS::IsDown,
                        EventT::Mouse(MouseButton2::LeftClick),
                    )]],
                ),
                (
                    BrushEvent::Remove,
                    vec![vec![(
                        EventS::IsDown,
                        EventT::Mouse(MouseButton2::RightClick),
                    )]],
                ),
                (
                    BrushEvent::InsertWalls,
                    vec![vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::I))]],
                ),
            ]),
        }
    }
}

impl Brush {
    pub async fn handle_events(&mut self, map: &mut Map, camera: &Camera2D) -> Result<()> {
        self.brush_events.update();

        if self.brush_events.pop(&BrushEvent::PickEmpty) {
            self.brush.tile_type = TileType::Empty;
        }
        if self.brush_events.pop(&BrushEvent::PickSmall) {
            self.brush.tile_type = TileType::Small;
        }
        if self.brush_events.pop(&BrushEvent::PickHalf) {
            self.brush.tile_type = TileType::Half;
        }
        if self.brush_events.pop(&BrushEvent::PickLarge) {
            self.brush.tile_type = TileType::Large;
        }
        if self.brush_events.pop(&BrushEvent::PickFull) {
            self.brush.tile_type = TileType::Full;
        }
        if self.brush_events.pop(&BrushEvent::RotateClockwise) {
            self.brush.rotate(1);
        }
        if self.brush_events.pop(&BrushEvent::RotateAntiClockwise) {
            self.brush.rotate(-1);
        }
        if self.brush_events.pop(&BrushEvent::SizeUp) {
            self.brush_size = self.brush_size.saturating_add(1);
        }
        if self.brush_events.pop(&BrushEvent::SizeDown) {
            self.brush_size = self.brush_size.saturating_sub(1);
        }

        if !self.brush.is_empty_or_full() {
            self.brush_size = 0;
        }

        let hoovered_hex = map
            .hex_layout
            .world_pos_to_hex(q2h(camera.screen_to_world(mouse_position().into())));

        if self.brush_events.pop(&BrushEvent::CloneTile)
            && let Some(tile) = map.tiles.get(&hoovered_hex)
        {
            self.brush = *tile;
        }
        if self.brush_events.pop(&BrushEvent::Insert) {
            if self.brush.is_empty_or_full() {
                for hex in hoovered_hex.range(self.brush_size) {
                    map.tiles.insert(hex, self.brush);
                }
            } else {
                map.tiles.insert(hoovered_hex, self.brush);
            }
            map.save_map().await?;
        }
        if self.brush_events.pop(&BrushEvent::Remove) {
            if self.brush.is_empty_or_full() {
                for hex in hoovered_hex.range(self.brush_size) {
                    map.tiles.remove(&hex);
                }
            } else {
                map.tiles.remove(&hoovered_hex);
            }
            map.save_map().await?;
        }

        if self.brush_events.pop(&BrushEvent::InsertWalls) {
            self.insert_walls(map);
            map.save_map().await?;
        }

        Ok(())
    }

    pub fn update(&mut self, map: &Map, camera: &Camera2D, dt: f32) -> Result<()> {
        let mouse_pos = h2q(map.hex_layout.hex_to_world_pos(
            map.hex_layout
                .world_pos_to_hex(q2h(camera.screen_to_world(mouse_position().into()))),
        ));

        let d = mouse_pos - self.brush_pos;
        if d.length() > 0.01 {
            self.brush_pos += d * self.smoothing_factor * dt;
        }

        Ok(())
    }

    pub fn draw(&self, map: &Map, theme: &Theme) {
        let hoovered_hex = map.hex_layout.world_pos_to_hex(q2h(self.brush_pos));
        let offset = q2h(self.brush_pos) - map.hex_layout.hex_to_world_pos(hoovered_hex);

        if self.brush.is_empty_or_full() {
            for hex in hoovered_hex.range(self.brush_size) {
                let pos = h2q(map.hex_layout.hex_to_world_pos(hex) + offset);
                self.brush.draw(
                    pos,
                    map.hex_size,
                    theme.color(ThemeColor::Light).with_alpha(0.5),
                    theme.color(ThemeColor::Normal).with_alpha(0.5),
                );
            }
        } else {
            let pos = h2q(map.hex_layout.hex_to_world_pos(hoovered_hex));
            self.brush.draw(
                pos,
                map.hex_size,
                theme.color(ThemeColor::Light).with_alpha(0.5),
                theme.color(ThemeColor::Normal).with_alpha(0.5),
            );
        }
    }

    pub fn insert_walls(&mut self, map: &mut Map) {
        let mc = map.tiles.clone();
        for (hex, tile) in mc.iter() {
            if tile.tile_type != TileType::Empty {
                continue;
            }
            for n0 in hex.all_neighbors() {
                if map.tiles.contains_key(&n0) {
                    continue;
                }

                let mut ns = [false; 6];
                for (v, n1) in n0.all_neighbors().into_iter().enumerate() {
                    if let Some(tile) = map.tiles.get(&n1)
                        && tile.tile_type == TileType::Empty
                    {
                        ns[v] = true;
                    }
                }

                let tile = match ns {
                    // 6
                    [true, true, true, true, true, true] => Tile::new(TileType::Empty, 0),

                    // 5
                    [true, true, true, true, true, false] => Tile::new(TileType::Empty, 0),
                    [true, true, true, true, false, true] => Tile::new(TileType::Empty, 0),
                    [true, true, true, false, true, true] => Tile::new(TileType::Empty, 0),
                    [true, true, false, true, true, true] => Tile::new(TileType::Empty, 0),
                    [true, false, true, true, true, true] => Tile::new(TileType::Empty, 0),
                    [false, true, true, true, true, true] => Tile::new(TileType::Empty, 0),

                    // 4
                    [true, true, true, true, false, false] => Tile::new(TileType::Small, 3),
                    [false, true, true, true, true, false] => Tile::new(TileType::Small, 4),
                    [false, false, true, true, true, true] => Tile::new(TileType::Small, 5),
                    [true, false, false, true, true, true] => Tile::new(TileType::Small, 0),
                    [true, true, false, false, true, true] => Tile::new(TileType::Small, 1),
                    [true, true, true, false, false, true] => Tile::new(TileType::Small, 2),

                    // 3
                    [true, true, true, false, false, false] => Tile::new(TileType::Half, 2),
                    [false, true, true, true, false, false] => Tile::new(TileType::Half, 3),
                    [false, false, true, true, true, false] => Tile::new(TileType::Half, 4),
                    [false, false, false, true, true, true] => Tile::new(TileType::Half, 5),
                    [true, false, false, false, true, true] => Tile::new(TileType::Half, 0),
                    [true, true, false, false, false, true] => Tile::new(TileType::Half, 1),

                    // 2
                    [true, true, false, false, false, false] => Tile::new(TileType::Large, 1),
                    [false, true, true, false, false, false] => Tile::new(TileType::Large, 2),
                    [false, false, true, true, false, false] => Tile::new(TileType::Large, 3),
                    [false, false, false, true, true, false] => Tile::new(TileType::Large, 4),
                    [false, false, false, false, true, true] => Tile::new(TileType::Large, 5),
                    [true, false, false, false, false, true] => Tile::new(TileType::Large, 0),

                    // 1
                    [true, false, false, false, false, false] => Tile::new(TileType::Full, 0),
                    [false, true, false, false, false, false] => Tile::new(TileType::Full, 0),
                    [false, false, true, false, false, false] => Tile::new(TileType::Full, 0),
                    [false, false, false, true, false, false] => Tile::new(TileType::Full, 0),
                    [false, false, false, false, true, false] => Tile::new(TileType::Full, 0),
                    [false, false, false, false, false, true] => Tile::new(TileType::Full, 0),

                    // 0
                    [false, false, false, false, false, false] => Tile::new(TileType::Full, 0),

                    _ => continue,
                };
                map.tiles.insert(n0, tile);
            }
        }
    }
}
