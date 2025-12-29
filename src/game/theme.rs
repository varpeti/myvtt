#![allow(dead_code)]

use anyhow::{Result, anyhow};
use macroquad::prelude::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Theme {
    Background,
    Text,

    Pillar,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Violet,
    Brown,

    HighLight,
    HighLight1,
}

impl Theme {
    pub fn u32(&self) -> u32 {
        match self {
            Theme::Background => Self::rosepine_bg,
            Theme::Text => Self::dragonWhite,

            Theme::Pillar => Self::rosepine_bg,
            Theme::Red => Self::autumnRed,
            Theme::Orange => Self::surimiOrange,
            Theme::Yellow => Self::autumnYellow,
            Theme::Green => Self::autumnGreen,
            Theme::Blue => Self::springBlue,
            Theme::Indigo => Self::oniViolet,
            Theme::Violet => Self::lotusViolet4,
            Theme::Brown => Self::winterRed,

            Theme::HighLight => Self::peachRed,
            Theme::HighLight1 => Self::waveRed,
        }
    }

    pub fn color(&self) -> Color {
        Color::from_hex(self.u32())
    }

    pub fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "Red" => Self::Red,
            "Orange" => Self::Orange,
            "Yellow" => Self::Yellow,
            "Green" => Self::Green,
            "Blue" => Self::Blue,
            "Indigo" => Self::Indigo,
            "Violet" => Self::Violet,
            "Brown" => Self::Brown,
            oth => return Err(anyhow!("Invalid Color: `{}`", oth)),
        })
    }
}

#[allow(non_upper_case_globals)]
impl Theme {
    /*
    Theme: Kanagawa
    Link: https://github.com/rebelot/kanagawa.nvim

    MIT License

        Copyright (c) 2021 Tommaso Laurenzi

        Permission is hereby granted, free of charge, to any person obtaining a copy
        of this software and associated documentation files (the "Software"), to deal
        in the Software without restriction, including without limitation the rights
        to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
        copies of the Software, and to permit persons to whom the Software is
        furnished to do so, subject to the following conditions:

        The above copyright notice and this permission notice shall be included in all
        copies or substantial portions of the Software.

        THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
        IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
        FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
        AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
        LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
        OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
        SOFTWARE.
    */
    const sumiInk0: u32 = 0x16161D; //  #16161D
    const sumiInk1: u32 = 0x181820; //  #181820
    const sumiInk2: u32 = 0x1a1a22; //  #1a1a22
    const sumiInk3: u32 = 0x1F1F28; //  #1F1F28
    const sumiInk4: u32 = 0x2A2A37; //  #2A2A37
    const sumiInk5: u32 = 0x363646; //  #363646
    const sumiInk6: u32 = 0x54546D; //  #54546D
    const waveBlue1: u32 = 0x223249; //  #223249
    const waveBlue2: u32 = 0x2D4F67; //  #2D4F67
    const winterGreen: u32 = 0x2B3328; //  #2B3328
    const winterYellow: u32 = 0x49443C; //  #49443C
    const winterRed: u32 = 0x43242B; //  #43242B
    const winterBlue: u32 = 0x252535; //  #252535
    const autumnGreen: u32 = 0x76946A; //  #76946A
    const autumnRed: u32 = 0xC34043; //  #C34043
    const autumnYellow: u32 = 0xDCA561; //  #DCA561
    const samuraiRed: u32 = 0xE82424; //  #E82424
    const roninYellow: u32 = 0xFF9E3B; //  #FF9E3B
    const waveAqua1: u32 = 0x6A9589; //  #6A9589
    const dragonBlue: u32 = 0x658594; //  #658594
    const oldWhite: u32 = 0xC8C093; //  #C8C093
    const fujiWhite: u32 = 0xDCD7BA; //  #DCD7BA
    const fujiGray: u32 = 0x727169; //  #727169
    const oniViolet: u32 = 0x957FB8; //  #957FB8
    const oniViolet2: u32 = 0xb8b4d0; //  #b8b4d0
    const crystalBlue: u32 = 0x7E9CD8; //  #7E9CD8
    const springViolet1: u32 = 0x938AA9; //  #938AA9
    const springViolet2: u32 = 0x9CABCA; //  #9CABCA
    const springBlue: u32 = 0x7FB4CA; //  #7FB4CA
    const lightBlue: u32 = 0xA3D4D5; //  #A3D4D5
    const waveAqua2: u32 = 0x7AA89F; //  #7AA89F
    const waveAqua3: u32 = 0x68AD99; //  #68AD99
    const waveAqua4: u32 = 0x7AA880; //  #7AA880
    const waveAqua5: u32 = 0x6CAF95; //  #6CAF95
    const springGreen: u32 = 0x98BB6C; //  #98BB6C
    const boatYellow1: u32 = 0x938056; //  #938056
    const boatYellow2: u32 = 0xC0A36E; //  #C0A36E
    const carpYellow: u32 = 0xE6C384; //  #E6C384
    const sakuraPink: u32 = 0xD27E99; //  #D27E99
    const waveRed: u32 = 0xE46876; //  #E46876
    const peachRed: u32 = 0xFF5D62; //  #FF5D62
    const surimiOrange: u32 = 0xFFA066; //  #FFA066
    const katanaGray: u32 = 0x717C7C; //  #717C7C
    const dragonBlack0: u32 = 0x0d0c0c; //  #0d0c0c
    const dragonBlack1: u32 = 0x12120f; //  #12120f
    const dragonBlack2: u32 = 0x1D1C19; //  #1D1C19
    const dragonBlack3: u32 = 0x181616; //  #181616
    const dragonBlack4: u32 = 0x282727; //  #282727
    const dragonBlack5: u32 = 0x393836; //  #393836
    const dragonBlack6: u32 = 0x625e5a; //  #625e5a
    const dragonWhite: u32 = 0xc5c9c5; //  #c5c9c5
    const dragonGreen: u32 = 0x87a987; //  #87a987
    const dragonGreen2: u32 = 0x8a9a7b; //  #8a9a7b
    const dragonPink: u32 = 0xa292a3; //  #a292a3
    const dragonOrange: u32 = 0xb6927b; //  #b6927b
    const dragonOrange2: u32 = 0xb98d7b; //  #b98d7b
    const dragonGray: u32 = 0xa6a69c; //  #a6a69c
    const dragonGray2: u32 = 0x9e9b93; //  #9e9b93
    const dragonGray3: u32 = 0x7a8382; //  #7a8382
    const dragonBlue2: u32 = 0x8ba4b0; //  #8ba4b0
    const dragonViolet: u32 = 0x8992a7; // #8992a7
    const dragonRed: u32 = 0xc4746e; //  #c4746e
    const dragonAqua: u32 = 0x8ea4a2; //  #8ea4a2
    const dragonAsh: u32 = 0x737c73; //  #737c73
    const dragonTeal: u32 = 0x949fb5; //  #949fb5
    const dragonYellow: u32 = 0xc4b28a; //  #c4b28a
    const lotusInk1: u32 = 0x545464; //  #545464
    const lotusInk2: u32 = 0x43436c; //  #43436c
    const lotusGray: u32 = 0xdcd7ba; //  #dcd7ba
    const lotusGray2: u32 = 0x716e61; //  #716e61
    const lotusGray3: u32 = 0x8a8980; //  #8a8980
    const lotusWhite0: u32 = 0xd5cea3; //  #d5cea3
    const lotusWhite1: u32 = 0xdcd5ac; //  #dcd5ac
    const lotusWhite2: u32 = 0xe5ddb0; //  #e5ddb0
    const lotusWhite3: u32 = 0xf2ecbc; //  #f2ecbc
    const lotusWhite4: u32 = 0xe7dba0; //  #e7dba0
    const lotusWhite5: u32 = 0xe4d794; //  #e4d794
    const lotusViolet1: u32 = 0xa09cac; //  #a09cac
    const lotusViolet2: u32 = 0x766b90; //  #766b90
    const lotusViolet3: u32 = 0xc9cbd1; //  #c9cbd1
    const lotusViolet4: u32 = 0x624c83; //  #624c83
    const lotusBlue1: u32 = 0xc7d7e0; //  #c7d7e0
    const lotusBlue2: u32 = 0xb5cbd2; //  #b5cbd2
    const lotusBlue3: u32 = 0x9fb5c9; //  #9fb5c9
    const lotusBlue4: u32 = 0x4d699b; //  #4d699b
    const lotusBlue5: u32 = 0x5d57a3; //  #5d57a3
    const lotusGreen: u32 = 0x6f894e; //  #6f894e
    const lotusGreen2: u32 = 0x6e915f; //  #6e915f
    const lotusGreen3: u32 = 0xb7d0ae; //  #b7d0ae
    const lotusPink: u32 = 0xb35b79; //  #b35b79
    const lotusOrange: u32 = 0xcc6d00; //  #cc6d00
    const lotusOrange2: u32 = 0xe98a00; //  #e98a00
    const lotusYellow: u32 = 0x77713f; //  #77713f
    const lotusYellow2: u32 = 0x836f4a; //  #836f4a
    const lotusYellow3: u32 = 0xde9800; //  #de9800
    const lotusYellow4: u32 = 0xf9d791; //  #f9d791
    const lotusRed: u32 = 0xc84053; //  #c84053
    const lotusRed2: u32 = 0xd7474b; //  #d7474b
    const lotusRed3: u32 = 0xe82424; //  #e82424
    const lotusRed4: u32 = 0xd9a594; //  #d9a594
    const lotusAqua: u32 = 0x597b75; //  #597b75
    const lotusAqua2: u32 = 0x5e857a; //  #5e857a
    const lotusTeal1: u32 = 0x4e8ca2; //  #4e8ca2
    const lotusTeal2: u32 = 0x6693bf; //  #6693bf
    const lotusTeal3: u32 = 0x5a7785; //  #5a7785
    const lotusCyan: u32 = 0xd7e3d8; //  #d7e3d8

    // I like it more 😅
    const rosepine_bg: u32 = 0x26233a; //  #26233a
}
