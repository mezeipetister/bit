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

pub struct Navbar {}

impl Navbar {
    pub fn new() -> Self {
        Navbar {}
    }
}

impl Component for Navbar {
    fn render(&self) -> Markup {
        html! {
            nav.navbar.is-light role="navigation" aria-label="main navigation" {
                .navbar-brand {
                    a.navbar-item href="/" {
                        span.title {"BIT"}
                    }
                    a.navbar-burger.burger role="button" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample" {
                        span aria-hidden="true" {}
                        span aria-hidden="true" {}
                        span aria-hidden="true" {}
                    }
                }
                #navbarBasicExample.navbar-menu {
                    .navbar-start {
                        a.navbar-item { "Repositories" }
                        a.navbar-item { "Documentation" }
                    }
                    .navbar-end {
                        .navbar-item {
                            .buttons {
                                a.button.is-light href="/settings" {
                                    span.icon.is-small {
                                        i.fa.fa-user-cog {}
                                    }
                                    span { "Settings" }
                                }
                                a.button.is-light href="/logout" accesskey="l" {
                                    span.icon.is-small {
                                        i.fa.fa-sign-out-alt {}
                                    }
                                    span { "Log out" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
