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

pub struct ViewAdminUserNew {}

impl ViewAdminUserNew {
    pub fn new() -> Self {
        ViewAdminUserNew {}
    }
}

impl View for ViewAdminUserNew {
    fn render(&self) -> Markup {
        html! {
            section.section {
                .container {
                    .columns.is-mobile {
                        .column {
                            p."is-size-3" {"Users"}
                            p.subtitle {"Add new user"}
                        }
                    }
                    form action="/admin/user/new" method="POST" {
                        .field {
                            label.label {"Username"}
                            .control {
                                input.input type="text" placeholder="e.g. johnsmith" name="id" autofocus?;
                            }
                        }
                        .field {
                            label.label {"Email"}
                            .control {
                                input.input type="text" placeholder="e.g. johnsmith@company.com" name="email";
                            }
                        }
                        .field {
                            label.label {"Name"}
                            .control {
                                input.input type="text" placeholder="e.g. John Smith" name="name";
                            }
                        }
                        .field {
                            .button-group {
                                button.button.is-info.is-outlined type="submit" {"Add user"}
                                a.button href="." {"Cancel"}
                            }
                        }
                    }
                }
            }
        }
    }
    fn render_success(&self) -> Markup {
        html! {}
    }
    fn render_error(&self) -> Markup {
        html! {}
    }
}
