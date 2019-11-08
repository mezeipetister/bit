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

pub struct ViewLogin {}

impl ViewLogin {
    pub fn new() -> Self {
        ViewLogin {}
    }
}

impl View for ViewLogin {
    fn render(&self) -> Markup {
        html! {
            section.section {
                .container {
                    p.has-text-centered.title.is-spaced {"Login"}
                    p.subtitle.has-text-centered."is-size-6" {
                        "BIT online financial"
                        br;
                        "and project information system"
                    }
                    form action="/login" method="POST" autocomplete="off" {
                        .columns {
                            .column."is-6-mobile"."is-offset-3-mobile"."is-6-desktop"."is-offset-3-desktop" {
                                .field {
                                    label.label { "Username" }
                                    .control {
                                        input.input type="text" name="username" placeholder="e.g. John Smith" autofocus? autocomplete="off";
                                    }
                                }
                                .field {
                                    label.label { "Password" }
                                    .control {
                                        input.input type="password" name="password" placeholder="strong password" autocomplete="off";
                                    }
                                }
                                .field {
                                    .control {
                                        .buttons {
                                            button.button.is-info.is-outlined type="submit" href="/login" { "Login" }
                                            a.button href="/login/reset_password" { "Forget password" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
