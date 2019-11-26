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
use crate::User;
use chrono::prelude::*;
use core_lib::prelude::DateCreated;
use maud::{html, Markup};
use storaget::*;

pub struct ViewAdminUser<'a, T>
where
    T: User,
{
    users: &'a Storage<T>,
}

impl<'a, T> ViewAdminUser<'a, T>
where
    T: User,
{
    pub fn new(users: &'a Storage<T>) -> Self {
        ViewAdminUser { users: users }
    }
}

impl<'a, T> View for ViewAdminUser<'a, T>
where
    T: User + DateCreated,
{
    fn render(&self) -> Markup {
        html! {
            section.section {
                .container {
                    .columns.is-mobile {
                        .column {
                            .content."is-size-3" { "Users" }
                        }
                        .column.has-text-right {
                            a.button href="/admin/user/new" {
                                span.icon.icon-small {
                                    i.far.fa-plus-square {}
                                }
                            }
                        }
                    }
                    table.table.is-striped {
                        @for user in self.users {
                            tr {
                                td {(user.get(|u| u.get_user_id().to_owned()))}
                                td {(user.get(|u| u.get_user_name().to_owned()))}
                                td {(user.get(|u| u.get_user_email().to_owned()))}
                                td {
                                    (format!("{}-{}-{} {}:{}",
                                    user.get(|u| u.get_date_created().year()),
                                    user.get(|u| u.get_date_created().month()),
                                    user.get(|u| u.get_date_created().day()),
                                    user.get(|u| u.get_date_created().hour()),
                                    user.get(|u| u.get_date_created().minute())))
                                }
                                // td {
                                //     .button-group {
                                //         a.button.is-small {
                                //             span.icon {
                                //                 i.fas.fa-key {}
                                //             }
                                //             span {"Reset password"}
                                //         }
                                //     }
                                // }
                            }
                        }
                    }
                }
            }
        }
    }
    fn render_error(&self) -> Markup {
        html! {}
    }
}
