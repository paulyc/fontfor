// FontFor: find fonts which can show a specified character
// Copyright (C) 2019 - 2020 7sDream <i@7sdre.am> and contributors
//
// This file is part of FontFor.
//
// FontFor is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use super::Render;

static MOON_CHARS: [[char; 2]; 5] =
    [['🌕', '🌕'], ['🌖', '🌔'], ['🌗', '🌓'], ['🌘', '🌒'], ['🌑', '🌑']];

pub struct MoonRender {
    pair_count: usize,
    multiplier: f64,
}

impl MoonRender {
    pub fn new() -> Self {
        let pair_count = MOON_CHARS.len();
        #[allow(clippy::cast_precision_loss)] // MOON_CHARS's length is small enough
        let multiplier = pair_count as f64 / 256.0;
        Self { pair_count, multiplier }
    }
}

impl Render for MoonRender {
    fn gray_to_char(&self, _up: u8, left: u8, gray: u8, right: u8, _down: u8) -> char {
        if gray == 0 {
            return MOON_CHARS[self.pair_count - 1][0];
        }

        #[allow(clippy::cast_sign_loss)] // because `255 - u8` must be non-neg
        #[allow(clippy::cast_possible_truncation)] // result small then MOON_CHARS's length
        let index = (f64::from(255 - gray) * self.multiplier).floor() as usize;

        if left < right {
            MOON_CHARS[index][1]
        } else {
            MOON_CHARS[index][0]
        }
    }
}
