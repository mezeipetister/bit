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
use crate::{User, UserObject};
use maud::{html, Markup};

pub struct ViewAdminUser<'a> {
    users: &'a Vec<UserObject>,
}

impl<'a> ViewAdminUser<'a> {
    pub fn new(users: &'a Vec<UserObject>) -> Self {
        ViewAdminUser { users: users }
    }
}

impl<'a> View for ViewAdminUser<'a> {
    fn render(&self) -> Markup {
        html! {
            section.section {
                .container {
                    .columns.is-mobile {
                        .column {
                            .content."is-size-3" { "Users" }
                        }
                        .column.has-text-right {
                            a.button href="/admin/users/new" {
                                span.icon.icon-small {
                                    i.far.fa-plus-square {}
                                }
                            }
                        }
                    }
                    table.table.is-striped {
                        @for user in self.users {
                            tr {
                                td {(user.get_user_id().unwrap_or("".to_owned()))}
                                td {(user.get_user_name().unwrap_or("".to_owned()))}
                                td {(user.get_user_email().unwrap_or("".to_owned()))}
                                td {
                                    .button-group {
                                        a.button.is-small {
                                            span.icon {
                                                i.fas.fa-key {}
                                            }
                                            span {"Reset password"}
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
    fn render_error(&self) -> Markup {
        html! {}
    }
}
