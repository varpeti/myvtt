#![allow(dead_code)]

use macroquad::prelude::Color;
use strum::VariantArray;
use strum_macros::{EnumString, IntoStaticStr, VariantArray};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, IntoStaticStr, VariantArray)]
pub enum Theme {
    SumiInk0,      //  #16161D
    SumiInk1,      //  #181820
    SumiInk2,      //  #1a1a22
    SumiInk3,      //  #1F1F28
    SumiInk4,      //  #2A2A37
    SumiInk5,      //  #363646
    SumiInk6,      //  #54546D
    WaveBlue1,     //  #223249
    WaveBlue2,     //  #2D4F67
    WinterGreen,   //  #2B3328
    WinterYellow,  //  #49443C
    WinterRed,     //  #43242B
    WinterBlue,    //  #252535
    AutumnGreen,   //  #76946A
    AutumnRed,     //  #C34043
    AutumnYellow,  //  #DCA561
    SamuraiRed,    //  #E82424
    RoninYellow,   //  #FF9E3B
    WaveAqua1,     //  #6A9589
    DragonBlue,    //  #658594
    OldWhite,      //  #C8C093
    FujiWhite,     //  #DCD7BA
    FujiGray,      //  #727169
    OniViolet,     //  #957FB8
    OniViolet2,    //  #b8b4d0
    CrystalBlue,   //  #7E9CD8
    SpringViolet1, //  #938AA9
    SpringViolet2, //  #9CABCA
    SpringBlue,    //  #7FB4CA
    LightBlue,     //  #A3D4D5
    WaveAqua2,     //  #7AA89F
    WaveAqua3,     //  #68AD99
    WaveAqua4,     //  #7AA880
    WaveAqua5,     //  #6CAF95
    SpringGreen,   //  #98BB6C
    BoatYellow1,   //  #938056
    BoatYellow2,   //  #C0A36E
    CarpYellow,    //  #E6C384
    SakuraPink,    //  #D27E99
    WaveRed,       //  #E46876
    PeachRed,      //  #FF5D62
    SurimiOrange,  //  #FFA066
    KatanaGray,    //  #717C7C
    DragonBlack0,  //  #0d0c0c
    DragonBlack1,  //  #12120f
    DragonBlack2,  //  #1D1C19
    DragonBlack3,  //  #181616
    DragonBlack4,  //  #282727
    DragonBlack5,  //  #393836
    DragonBlack6,  //  #625e5a
    DragonWhite,   //  #c5c9c5
    DragonGreen,   //  #87a987
    DragonGreen2,  //  #8a9a7b
    DragonPink,    //  #a292a3
    DragonOrange,  //  #b6927b
    DragonOrange2, //  #b98d7b
    DragonGray,    //  #a6a69c
    DragonGray2,   //  #9e9b93
    DragonGray3,   //  #7a8382
    DragonBlue2,   //  #8ba4b0
    DragonViolet,  //  #8992a7
    DragonRed,     //  #c4746e
    DragonAqua,    //  #8ea4a2
    DragonAsh,     //  #737c73
    DragonTeal,    //  #949fb5
    DragonYellow,  //  #c4b28a
    LotusInk1,     //  #545464
    LotusInk2,     //  #43436c
    LotusGray,     //  #dcd7ba
    LotusGray2,    //  #716e61
    LotusGray3,    //  #8a8980
    LotusWhite0,   //  #d5cea3
    LotusWhite1,   //  #dcd5ac
    LotusWhite2,   //  #e5ddb0
    LotusWhite3,   //  #f2ecbc
    LotusWhite4,   //  #e7dba0
    LotusWhite5,   //  #e4d794
    LotusViolet1,  //  #a09cac
    LotusViolet2,  //  #766b90
    LotusViolet3,  //  #c9cbd1
    LotusViolet4,  //  #624c83
    LotusBlue1,    //  #c7d7e0
    LotusBlue2,    //  #b5cbd2
    LotusBlue3,    //  #9fb5c9
    LotusBlue4,    //  #4d699b
    LotusBlue5,    //  #5d57a3
    LotusGreen,    //  #6f894e
    LotusGreen2,   //  #6e915f
    LotusGreen3,   //  #b7d0ae
    LotusPink,     //  #b35b79
    LotusOrange,   //  #cc6d00
    LotusOrange2,  //  #e98a00
    LotusYellow,   //  #77713f
    LotusYellow2,  //  #836f4a
    LotusYellow3,  //  #de9800
    LotusYellow4,  //  #f9d791
    LotusRed,      //  #c84053
    LotusRed2,     //  #d7474b
    LotusRed3,     //  #e82424
    LotusRed4,     //  #d9a594
    LotusAqua,     //  #597b75
    LotusAqua2,    //  #5e857a
    LotusTeal1,    //  #4e8ca2
    LotusTeal2,    //  #6693bf
    LotusTeal3,    //  #5a7785
    LotusCyan,     //  #d7e3d8

    Background,   //  #26233a
    PillarOffset, //  #4c4674
}

impl Theme {
    pub fn u32(&self) -> u32 {
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
        match self {
            Theme::SumiInk0 => 0x16161D,      //  #16161D
            Theme::SumiInk1 => 0x181820,      //  #181820
            Theme::SumiInk2 => 0x1a1a22,      //  #1a1a22
            Theme::SumiInk3 => 0x1F1F28,      //  #1F1F28
            Theme::SumiInk4 => 0x2A2A37,      //  #2A2A37
            Theme::SumiInk5 => 0x363646,      //  #363646
            Theme::SumiInk6 => 0x54546D,      //  #54546D
            Theme::WaveBlue1 => 0x223249,     //  #223249
            Theme::WaveBlue2 => 0x2D4F67,     //  #2D4F67
            Theme::WinterGreen => 0x2B3328,   //  #2B3328
            Theme::WinterYellow => 0x49443C,  //  #49443C
            Theme::WinterRed => 0x43242B,     //  #43242B
            Theme::WinterBlue => 0x252535,    //  #252535
            Theme::AutumnGreen => 0x76946A,   //  #76946A
            Theme::AutumnRed => 0xC34043,     //  #C34043
            Theme::AutumnYellow => 0xDCA561,  //  #DCA561
            Theme::SamuraiRed => 0xE82424,    //  #E82424
            Theme::RoninYellow => 0xFF9E3B,   //  #FF9E3B
            Theme::WaveAqua1 => 0x6A9589,     //  #6A9589
            Theme::DragonBlue => 0x658594,    //  #658594
            Theme::OldWhite => 0xC8C093,      //  #C8C093
            Theme::FujiWhite => 0xDCD7BA,     //  #DCD7BA
            Theme::FujiGray => 0x727169,      //  #727169
            Theme::OniViolet => 0x957FB8,     //  #957FB8
            Theme::OniViolet2 => 0xb8b4d0,    //  #b8b4d0
            Theme::CrystalBlue => 0x7E9CD8,   //  #7E9CD8
            Theme::SpringViolet1 => 0x938AA9, //  #938AA9
            Theme::SpringViolet2 => 0x9CABCA, //  #9CABCA
            Theme::SpringBlue => 0x7FB4CA,    //  #7FB4CA
            Theme::LightBlue => 0xA3D4D5,     //  #A3D4D5
            Theme::WaveAqua2 => 0x7AA89F,     //  #7AA89F
            Theme::WaveAqua3 => 0x68AD99,     //  #68AD99
            Theme::WaveAqua4 => 0x7AA880,     //  #7AA880
            Theme::WaveAqua5 => 0x6CAF95,     //  #6CAF95
            Theme::SpringGreen => 0x98BB6C,   //  #98BB6C
            Theme::BoatYellow1 => 0x938056,   //  #938056
            Theme::BoatYellow2 => 0xC0A36E,   //  #C0A36E
            Theme::CarpYellow => 0xE6C384,    //  #E6C384
            Theme::SakuraPink => 0xD27E99,    //  #D27E99
            Theme::WaveRed => 0xE46876,       //  #E46876
            Theme::PeachRed => 0xFF5D62,      //  #FF5D62
            Theme::SurimiOrange => 0xFFA066,  //  #FFA066
            Theme::KatanaGray => 0x717C7C,    //  #717C7C
            Theme::DragonBlack0 => 0x0d0c0c,  //  #0d0c0c
            Theme::DragonBlack1 => 0x12120f,  //  #12120f
            Theme::DragonBlack2 => 0x1D1C19,  //  #1D1C19
            Theme::DragonBlack3 => 0x181616,  //  #181616
            Theme::DragonBlack4 => 0x282727,  //  #282727
            Theme::DragonBlack5 => 0x393836,  //  #393836
            Theme::DragonBlack6 => 0x625e5a,  //  #625e5a
            Theme::DragonWhite => 0xc5c9c5,   //  #c5c9c5
            Theme::DragonGreen => 0x87a987,   //  #87a987
            Theme::DragonGreen2 => 0x8a9a7b,  //  #8a9a7b
            Theme::DragonPink => 0xa292a3,    //  #a292a3
            Theme::DragonOrange => 0xb6927b,  //  #b6927b
            Theme::DragonOrange2 => 0xb98d7b, //  #b98d7b
            Theme::DragonGray => 0xa6a69c,    //  #a6a69c
            Theme::DragonGray2 => 0x9e9b93,   //  #9e9b93
            Theme::DragonGray3 => 0x7a8382,   //  #7a8382
            Theme::DragonBlue2 => 0x8ba4b0,   //  #8ba4b0
            Theme::DragonViolet => 0x8992a7,  //  #8992a7
            Theme::DragonRed => 0xc4746e,     //  #c4746e
            Theme::DragonAqua => 0x8ea4a2,    //  #8ea4a2
            Theme::DragonAsh => 0x737c73,     //  #737c73
            Theme::DragonTeal => 0x949fb5,    //  #949fb5
            Theme::DragonYellow => 0xc4b28a,  //  #c4b28a
            Theme::LotusInk1 => 0x545464,     //  #545464
            Theme::LotusInk2 => 0x43436c,     //  #43436c
            Theme::LotusGray => 0xdcd7ba,     //  #dcd7ba
            Theme::LotusGray2 => 0x716e61,    //  #716e61
            Theme::LotusGray3 => 0x8a8980,    //  #8a8980
            Theme::LotusWhite0 => 0xd5cea3,   //  #d5cea3
            Theme::LotusWhite1 => 0xdcd5ac,   //  #dcd5ac
            Theme::LotusWhite2 => 0xe5ddb0,   //  #e5ddb0
            Theme::LotusWhite3 => 0xf2ecbc,   //  #f2ecbc
            Theme::LotusWhite4 => 0xe7dba0,   //  #e7dba0
            Theme::LotusWhite5 => 0xe4d794,   //  #e4d794
            Theme::LotusViolet1 => 0xa09cac,  //  #a09cac
            Theme::LotusViolet2 => 0x766b90,  //  #766b90
            Theme::LotusViolet3 => 0xc9cbd1,  //  #c9cbd1
            Theme::LotusViolet4 => 0x624c83,  //  #624c83
            Theme::LotusBlue1 => 0xc7d7e0,    //  #c7d7e0
            Theme::LotusBlue2 => 0xb5cbd2,    //  #b5cbd2
            Theme::LotusBlue3 => 0x9fb5c9,    //  #9fb5c9
            Theme::LotusBlue4 => 0x4d699b,    //  #4d699b
            Theme::LotusBlue5 => 0x5d57a3,    //  #5d57a3
            Theme::LotusGreen => 0x6f894e,    //  #6f894e
            Theme::LotusGreen2 => 0x6e915f,   //  #6e915f
            Theme::LotusGreen3 => 0xb7d0ae,   //  #b7d0ae
            Theme::LotusPink => 0xb35b79,     //  #b35b79
            Theme::LotusOrange => 0xcc6d00,   //  #cc6d00
            Theme::LotusOrange2 => 0xe98a00,  //  #e98a00
            Theme::LotusYellow => 0x77713f,   //  #77713f
            Theme::LotusYellow2 => 0x836f4a,  //  #836f4a
            Theme::LotusYellow3 => 0xde9800,  //  #de9800
            Theme::LotusYellow4 => 0xf9d791,  //  #f9d791
            Theme::LotusRed => 0xc84053,      //  #c84053
            Theme::LotusRed2 => 0xd7474b,     //  #d7474b
            Theme::LotusRed3 => 0xe82424,     //  #e82424
            Theme::LotusRed4 => 0xd9a594,     //  #d9a594
            Theme::LotusAqua => 0x597b75,     //  #597b75
            Theme::LotusAqua2 => 0x5e857a,    //  #5e857a
            Theme::LotusTeal1 => 0x4e8ca2,    //  #4e8ca2
            Theme::LotusTeal2 => 0x6693bf,    //  #6693bf
            Theme::LotusTeal3 => 0x5a7785,    //  #5a7785
            Theme::LotusCyan => 0xd7e3d8,     //  #d7e3d8

            Theme::Background => 0x26233a,   //  #26233a
            Theme::PillarOffset => 0x4c4674, //  #4c4674 For each chanel: (Target/0xff)/(Texture/0xff) * (0xff)
        }
    }

    pub fn color(&self) -> Color {
        Color::from_hex(self.u32())
    }

    pub fn next(&self) -> Self {
        let variants = Self::VARIANTS;
        let current_index = variants.iter().position(|&x| x == *self).unwrap();
        variants[(current_index + 1) % variants.len()]
    }

    pub fn previous(&self) -> Self {
        let variants = Self::VARIANTS;
        let current_index = variants.iter().position(|&x| x == *self).unwrap();
        variants[(current_index + variants.len() - 1) % variants.len()]
    }
}
