use std::collections::HashMap;

use anyhow::Result;
use hexx::Hex;
use macroquad::prelude::*;

use crate::game::{
    events::{Event, EventS, EventT, Events, MouseButton2},
    map::{Map, h2q, q2h, tile::Tile},
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
    to_fade: HashMap<Hex, f32>,
    fade_factor: f32,
    brush: Tile,
    brush_size: u32,
    brush_max_size: u32,
    brush_events: Events<BrushEvent>,
}

impl Default for Brush {
    fn default() -> Self {
        Self {
            to_fade: HashMap::new(),
            fade_factor: 5.,
            brush: Tile::Empty,
            brush_size: 0,
            brush_max_size: 16,
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
                        EventS::IsPressed,
                        EventT::Mouse(MouseButton2::LeftClick),
                    )]],
                ),
                (
                    BrushEvent::Remove,
                    vec![vec![(
                        EventS::IsPressed,
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
            self.brush = Tile::Empty;
        }
        if self.brush_events.pop(&BrushEvent::PickSmall) {
            self.brush = Tile::Small { rotation: 0 };
        }
        if self.brush_events.pop(&BrushEvent::PickHalf) {
            self.brush = Tile::Half { rotation: 0 };
        }
        if self.brush_events.pop(&BrushEvent::PickLarge) {
            self.brush = Tile::Large { rotation: 0 };
        }
        if self.brush_events.pop(&BrushEvent::PickFull) {
            self.brush = Tile::Full;
        }
        if self.brush_events.pop(&BrushEvent::RotateClockwise) {
            self.brush.rotate(1);
        }
        if self.brush_events.pop(&BrushEvent::RotateAntiClockwise) {
            self.brush.rotate(-1);
        }
        if self.brush_events.pop(&BrushEvent::SizeUp) {
            if self.brush_size == self.brush_max_size {
                self.brush_size = self.brush_max_size;
            } else {
                self.brush_size += 1;
            }
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
        }
        if self.brush_events.pop(&BrushEvent::Remove) {
            if self.brush.is_empty_or_full() {
                for hex in hoovered_hex.range(self.brush_size) {
                    map.tiles.remove(&hex);
                }
            } else {
                map.tiles.remove(&hoovered_hex);
            }
        }

        if self.brush_events.pop(&BrushEvent::InsertWalls) {
            self.insert_walls(map);
        }

        Ok(())
    }

    pub fn update(&mut self, map: &Map, camera: &Camera2D, dt: f32) -> Result<()> {
        self.to_fade.retain(|_, alpha| {
            *alpha -= self.fade_factor * dt;
            *alpha > 0.
        });

        let hoovered_hex = map
            .hex_layout
            .world_pos_to_hex(q2h(camera.screen_to_world(mouse_position().into())));

        if self.brush.is_empty_or_full() {
            for hex in hoovered_hex.range(self.brush_size) {
                self.to_fade.insert(hex, 1.);
            }
        } else {
            self.to_fade.insert(hoovered_hex, 1.);
        }

        Ok(())
    }

    pub fn draw(&self, map: &Map, theme: &Theme) {
        for (&hex, alpha) in self.to_fade.iter() {
            let pos = h2q(map.hex_layout.hex_to_world_pos(hex));
            self.brush.draw(
                pos,
                map.hex_size,
                theme.color(ThemeColor::Light).with_alpha(0.5 * alpha),
                theme.color(ThemeColor::Normal).with_alpha(0.5 * alpha),
                &map.tile_variants,
            );
        }
    }

    pub fn insert_walls(&mut self, map: &mut Map) {
        let mc = map.tiles.clone();
        for (hex, tile) in mc.iter() {
            if *tile != Tile::Empty {
                continue;
            }
            for n0 in hex.all_neighbors() {
                if map.tiles.contains_key(&n0) {
                    continue;
                }

                let mut ns = [false; 6];
                for (v, n1) in n0.all_neighbors().into_iter().enumerate() {
                    if let Some(tile) = map.tiles.get(&n1)
                        && *tile == Tile::Empty
                    {
                        ns[v] = true;
                    }
                }

                let tile = match ns {
                    // 6
                    [true, true, true, true, true, true] => Tile::Empty,

                    // 5
                    [true, true, true, true, true, false] => Tile::Empty,
                    [true, true, true, true, false, true] => Tile::Empty,
                    [true, true, true, false, true, true] => Tile::Empty,
                    [true, true, false, true, true, true] => Tile::Empty,
                    [true, false, true, true, true, true] => Tile::Empty,
                    [false, true, true, true, true, true] => Tile::Empty,

                    // 4
                    [true, true, true, true, false, false] => Tile::Small { rotation: 3 },
                    [false, true, true, true, true, false] => Tile::Small { rotation: 4 },
                    [false, false, true, true, true, true] => Tile::Small { rotation: 5 },
                    [true, false, false, true, true, true] => Tile::Small { rotation: 0 },
                    [true, true, false, false, true, true] => Tile::Small { rotation: 1 },
                    [true, true, true, false, false, true] => Tile::Small { rotation: 2 },

                    // 3
                    [true, true, true, false, false, false] => Tile::Half { rotation: 2 },
                    [false, true, true, true, false, false] => Tile::Half { rotation: 3 },
                    [false, false, true, true, true, false] => Tile::Half { rotation: 4 },
                    [false, false, false, true, true, true] => Tile::Half { rotation: 5 },
                    [true, false, false, false, true, true] => Tile::Half { rotation: 0 },
                    [true, true, false, false, false, true] => Tile::Half { rotation: 1 },

                    // 2
                    [true, true, false, false, false, false] => Tile::Large { rotation: 5 },
                    [false, true, true, false, false, false] => Tile::Large { rotation: 0 },
                    [false, false, true, true, false, false] => Tile::Large { rotation: 1 },
                    [false, false, false, true, true, false] => Tile::Large { rotation: 2 },
                    [false, false, false, false, true, true] => Tile::Large { rotation: 3 },
                    [true, false, false, false, false, true] => Tile::Large { rotation: 4 },

                    // 1
                    [true, false, false, false, false, false] => Tile::Full,
                    [false, true, false, false, false, false] => Tile::Full,
                    [false, false, true, false, false, false] => Tile::Full,
                    [false, false, false, true, false, false] => Tile::Full,
                    [false, false, false, false, true, false] => Tile::Full,
                    [false, false, false, false, false, true] => Tile::Full,

                    // 0
                    [false, false, false, false, false, false] => Tile::Full,

                    _ => continue,
                };
                map.tiles.insert(n0, tile);
            }
        }
    }
}
