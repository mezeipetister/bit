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

pub struct TabBar {}

impl TabBar {
    pub fn new() -> Self {
        TabBar {}
    }
}

impl Component for TabBar {
    fn render(&self) -> Markup {
        html! {
            section.section {
                .container {
                    .tabs.is-centered {
                        ul {
                            li.is-active {
                                a href="/dashboard" {
                                    span.icon.is-small {
                                        i.fa.fa-tachometer-alt aria-hidden="true" {}
                                    }
                                    span { "Dashboard" }
                                }
                            }
                            li {
                                a href="/events" {
                                    span.icon.is-small {
                                        i.fa.fa-history aria-hidden="true" {}
                                    }
                                    span { "Events" }
                                }
                            }
                            li {
                                a href="/accounts" {
                                    span.icon.is-small {
                                        i.fa.fa-list-alt aria-hidden="true" {}
                                    }
                                    span { "Accounts" }
                                }
                            }
                            // li {
                            //     a href="/projects" {
                            //         span.icon.is-small {
                            //             i.fa.fa-project-diagram aria-hidden="true" {}
                            //         }
                            //         span { "Projects" }
                            //     }
                            // }
                            // li {
                            //     a href="/team" {
                            //         span.icon.is-small {
                            //             i.fa.fa-user-friends aria-hidden="true" {}
                            //         }
                            //         span { "Team" }
                            //     }
                            // }
                        }
                    }
                }
            }
        }
    }
}
