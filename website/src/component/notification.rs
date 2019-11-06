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
use rocket::request::FlashMessage;

pub struct Notification<'a, 'r> {
    msg: Option<FlashMessage<'a, 'r>>,
}

impl<'a, 'r> Notification<'a, 'r> {
    pub fn new(msg: Option<FlashMessage<'a, 'r>>) -> Self {
        Notification { msg }
    }
    fn get_class(&self) -> String {
        match &self.msg {
            Some(msg) => format!("is-{}", msg.name()),
            None => "".into(),
        }
    }
}

impl<'a, 'r> Component for Notification<'a, 'r> {
    fn render(&self) -> Markup {
        html! {
            @if let Some(msg) = &self.msg {
                .container {
                    .columns {
                        .column."is-6-desktop"."is-offset-3-desktop" {
                            .notification.(self.get_class()) {
                                // button.delete {}
                                {(msg.msg())}
                            }
                        }
                    }
                }
            }
        }
    }
}
