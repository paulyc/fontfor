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

use crate::{
    font::{GetValueByLang, SortedFamilies},
    preview::terminal::render::RenderResult,
};

pub struct State<'a> {
    pub(super) families: SortedFamilies<'a>,
    pub(super) names: Vec<&'a str>,
    pub(super) name_max_width: usize,
    pub(super) index: usize,
    pub(super) cache: Vec<Option<RenderResult>>,
}

impl<'a> State<'a> {
    pub fn new(families: SortedFamilies<'a>) -> Self {
        let name_max_width =
            families.iter().map(|f| f.default_name_width).max().unwrap_or_default();
        let names = families.iter().map(|f| *f.name.get_default()).collect();
        let cache = vec![None; families.len()];
        Self { families, names, index: 0, name_max_width, cache }
    }

    pub fn current_name(&self) -> &'a str {
        self.names[self.index]
    }
}
