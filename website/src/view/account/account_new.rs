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
// use core_lib::Account;
use maud::{html, Markup};
// use storaget::*;

pub struct ViewAccountNew {}

impl ViewAccountNew {
    pub fn new() -> Self {
        ViewAccountNew {}
    }
}

impl View for ViewAccountNew {
    fn render(&self) -> Markup {
        html! {
            section.section {
                .container.content {
                    h2.title {"New account"}
                    form method="POST" action="/accounts/new" {
                        .field {
                            label.label {"Account ID"}
                            .control {
                                input.input type="text" name="account_id" placeholder="e.g.: 161" autofocus?;
                            }
                        }
                        .field {
                            label.label {"Account name"}
                            .control {
                                input.input type="text" name="account_name" placeholder="e.g.: Investment";
                            }
                        }
                        .field {
                            label.label {"Account description"}
                            .control {
                                input.input type="text" name="account_description" placeholder="e.g.: Hello world";
                            }
                        }
                        .buttons {
                            button.button.is-primary.is-outlined type="submit" {"Save"}
                            a.button href="." {"Cancel"}
                        }
                    }
                }
            }
        }
    }
}
