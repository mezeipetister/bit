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

use crate::core_lib::storage::StorageObject;
use crate::core_lib::user::User;
use crate::view::View;
use maud::{html, Markup};

pub struct ViewSettings<'a, T>
where
    T: 'a + User + StorageObject,
{
    user: &'a T,
}

impl<'a, T> ViewSettings<'a, T>
where
    T: 'a + User + StorageObject,
{
    pub fn new(user: &'a T) -> Self {
        ViewSettings { user }
    }
}

impl<'a, T> View for ViewSettings<'a, T>
where
    T: 'a + User + StorageObject,
{
    fn render(&self) -> Markup {
        html! {
            section.section {
                .container {
                    .columns."is-mobile" {
                        .column {
                            p."is-size-3" {"Settings"}
                        }
                    }
                    .field {
                        label.label {"Username"}
                        .control {
                            input.input."is-static" type="text" placeholder="e.g. johnsmith" value=(self.user.get_user_id()) readonly?;
                        }
                    }
                    form method="POST" action="/settings" {
                        .field {
                            label.label {"Email"}
                            .control {
                                input.input type="email" name="email" placeholder="e.g. johnsmith@gmail.com" value=(self.user.get_user_email());
                            }
                        }
                        .field {
                            label.label {"Name"}
                            .control {
                                input.input type="text" name="name" placeholder="e.g. John Smith" value=(self.user.get_user_name());
                            }
                        }
                        .field {
                            .buttons {
                                button.button."is-info"."is-outlined" type="submit" {"Save"}
                                a.button."is-info"."is-outlined" href="/settings/new_password" {"Change password"}
                                a.button."is-info"."is-outlined" href="." {"Cancel"}
                            }
                        }
                    }
                }
            }
        }
    }
}
