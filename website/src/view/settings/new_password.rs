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

pub struct ViewNewPassword {}

impl ViewNewPassword {
    pub fn new() -> Self {
        ViewNewPassword {}
    }
}

impl View for ViewNewPassword {
    fn render(&self) -> Markup {
        html! {
            section.section {
                .container {
                    .columns."is-mobile" {
                        .column {
                            p."is-size-3" {"Settings / Password"}
                            p.subtitle {"Please type your new password twice."}
                        }
                    }
                    form method="POST" action="/settings/new_password" {
                        .field {
                            label.label {"New password"}
                            .control {
                                input.input type="password" name="password1" placeholder="e.g. strong_password" autofocus?;
                            }
                        }
                        .field {
                            label.label {"New password again"}
                            .control {
                                input.input type="password" name="password2" placeholder="e.g. strong_password";
                            }
                        }
                        .field {
                            .button-group {
                                button.button."is-info"."is-outlined" type="submit" {"Change password"}
                                a.button href="." {"Cancel"}
                            }
                        }
                    }
                }
            }
        }
    }
    fn render_error(&self) -> Markup {
        html! {
            section.section {
                .container {
                    p."has-text-centered"."title"."is-spaced" {"Oooops! Error!"}
                    p.subtitle."has-text-centered"."is-size-6" {"You need stronger password."}
                    .columns."is-mobile" {
                        .column."is-6-mobile"."is-offset-3-mobile"."is-4-desktop"."is-offset-4-desktop" {
                            .field {
                                .control."has-text-centered" {
                                    ."button-group" {
                                        a.button href="/settings/new_password" {"Try again"}
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
