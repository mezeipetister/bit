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

pub struct ViewPasswordReset {}

impl ViewPasswordReset {
    pub fn new() -> Self {
        ViewPasswordReset {}
    }
}

impl View for ViewPasswordReset {
    fn render(&self) -> Markup {
        html! {
            section.section {
                .container {
                    p.has-text-centered.title.is-spaced {"Reset password"}
                    p.subtitle.has-text-centered."is-size-6" {
                        "Please type your email address."
                        br;
                        "We are going to create a new password and"
                        br;
                        "send it to your email address."
                    }
                    form action="/login/reset_password" method="POST" {
                        .column."is-6-mobile"."is-offset-3-mobile"."is-4-desktop"."is-offset-4-desktop" {
                            .field {
                                .control {
                                    input.input type="text" name="email" placeholder="e.g. john.smith@company.com" autofocus?;
                                }
                            }
                            .field {
                                .control.has-text-centered {
                                    .button-group {
                                        button.button type="submit" { "Send me a new password" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn render_success(&self) -> Markup {
        html! {
            section.section {
                .container {
                    p.has-text-centered.title.is-spaced {"Reset password"}
                    p.subtitle.has-text-centered."is-size-6" {
                        "Your password has been sent!"
                        br;
                        "Please check your inbox."
                    }
                    .columns.is-mobile {
                        .column."is-6-mobile"."is-offset-3-mobile"."is-4-desktop"."is-offset-4-desktop" {
                            .field {
                                .control.has-text-centered {
                                    form action="/login" method="GET" {
                                        button.button type="submit" autofocus? { "Login" }
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
        html! {
            section.section {
                .container {
                    p.has-text-centered.title.is-spaced {"Reset password"}
                    p.subtitle.has-text-centered."is-size-6" {
                        "Oooops!"
                        br;
                        "We do not know your email address."
                    }
                    .columns.is-mobile {
                        .column."is-6-mobile"."is-offset-3-mobile"."is-4-desktop"."is-offset-4-desktop" {
                            .field {
                                .control.has-text-centered {
                                    a.button href="/login/reset_password" { "Try again" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
