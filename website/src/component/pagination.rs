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

use crate::component::Component;
use maud::{html, Markup};

pub struct Pagination {
    pages: Vec<u32>,
    previous: u32,
    next: u32,
}

impl Pagination {
    pub fn new(pages: Vec<u32>, previous: u32, next: u32) -> Self {
        Pagination {
            pages,
            previous,
            next,
        }
    }
}

impl Component for Pagination {
    fn render(&self) -> Markup {
        html! {
            nav.pagination role="navigation" aria-label="pagination" {
                a.pagination-previous title="Previous page" href=(self.previous) disabled? {"Previous"}
                a.pagination-next title="Next page" href=(self.next) {"Next"}
                ul.pagination-list {
                    @for item in &self.pages {
                        li {
                            a.pagination-link.is-current href=(format!("{}", item)) aria-label=(format!("Page {}", item)) aria-current="page" {(item)}
                        }
                    }
                }
            }
        }
    }
}
