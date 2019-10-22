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

use crate::component::*;
use maud::{html, Markup, DOCTYPE};

pub struct Layout<'a> {
    pub title: Option<&'a str>,
    pub meta_keywords: Option<Vec<&'a str>>,
    pub meta_description: Option<&'a str>,
    pub meta_author: Option<&'a str>,
    pub meta_language: Option<&'a str>,
    pub meta_robots: Option<&'a str>,
    pub meta_designer: Option<&'a str>,
    pub meta_publisher: Option<&'a str>,
    pub navbar: Option<Markup>,
    pub tabbar: Option<Markup>,
    pub footer: Option<Markup>,
}

impl<'a> Layout<'a> {
    pub fn new() -> Self {
        Layout {
            title: None,
            meta_keywords: None,
            meta_description: None,
            meta_author: None,
            meta_language: None,
            meta_robots: None,
            meta_designer: None,
            meta_publisher: None,
            navbar: Some(Navbar::default()),
            tabbar: Some(TabBar::default()),
            footer: Some(Footer::default()),
        }
    }
    pub fn set_title(&mut self, title: &'a str) -> &'a mut Layout {
        self.title = Some(title);
        self
    }
    pub fn set_empty(&mut self) -> &'a mut Layout {
        self.navbar = None;
        self.tabbar = None;
        self.footer = None;
        self
    }
    pub fn disable_tabbar(&mut self) -> &'a mut Layout {
        self.tabbar = None;
        self
    }
    pub fn disable_navbar(&mut self) -> &'a mut Layout {
        self.navbar = None;
        self
    }
    pub fn disable_footer(&mut self) -> &'a mut Layout {
        self.footer = None;
        self
    }
    pub fn render(&self, body: Markup) -> Markup {
        html! {
            (DOCTYPE)
            html {
                head {
                    meta charset="utf-8" /
                    meta name="viewport" content="width=device-width, initial-scale=1";
                    title {(self.title.unwrap_or("TITLE"))}
                    link rel="stylesheet" type="text/css" href="/static/style.css";
                    link rel="icon" type="image/x-icon" href="data:image/x-icon;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQEAYAAABPYyMiAAAABmJLR0T///////8JWPfcAAAACXBIWXMAAABIAAAASABGyWs+AAAAF0lEQVRIx2NgGAWjYBSMglEwCkbBSAcACBAAAeaR9cIAAAAASUVORK5CYII=";
                }
                body {
                    (self.navbar.as_ref().unwrap_or(&html!{}))
                    (self.tabbar.as_ref().unwrap_or(&html!{}))
                    (body)
                    (self.footer.as_ref().unwrap_or(&html!{}))
                    script defer? src="/static/script.js" {}
                    script defer? src="/static/fa.js" {}
                    script defer? src="/static/shortcut.js" {}
                }
            }
        }
    }
}
