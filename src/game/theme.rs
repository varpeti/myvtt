#![allow(dead_code)]

use macroquad::color::Color;

pub struct Theme {
    colors: [u32; 5],
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            colors: [
                0x2e2924, // #2e2924
                0x7d6c5e, // #7d6c5e
                0xc0b18c, // #c0b18c
                0xd8c3a2, // #d8c3a2
                0xf2e7d4, // #f2e7d4
            ],
        }
    }
}

impl Theme {
    pub fn u32(&self, color: ThemeColor) -> u32 {
        self.colors[color as isize as usize]
    }

    pub fn color(&self, color: ThemeColor) -> Color {
        Color::from_hex(self.colors[color as isize as usize])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeColor {
    Darker,
    Dark,
    Normal,
    Light,
    Lighter,
}
