// Copyright (C) 2019 Peter Mezei
//
// This file is part of Project A.
//
// Project A is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// Project A is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Project A.  If not, see <http://www.gnu.org/licenses/>.

use crate::view::View;
use maud::{html, Markup};

pub struct ViewIndex<'a> {
    name: &'a str,
}

impl<'a> ViewIndex<'a> {
    pub fn new() -> Self {
        ViewIndex { name: "" }
    }
    pub fn set_name(&mut self, name: &'a str) -> &mut Self {
        self.name = name;
        self
    }
}

impl<'a> View for ViewIndex<'a> {
    fn render(&self) -> Markup {
        html! {
            section.section {
                .container {
                    p."title"."is-3" { "Hello World "}
                    p."subtitle"."is-4" { "Your name is: "(self.name)"" }
                    p.content {
                        "lorem ipsum dolorem set ami"
                    }
                }
            }
        }
    }
}
