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

pub struct Footer {}

impl Component for Footer {
    fn default() -> Markup {
        html! {
            footer.footer {
                .content.has-text-centered {
                    p.small.has-text-weight-light {
                        strong {"BIT"} " by " a href="https://github.com/mezeipetister" target="_blank" accesskey="r" {"Peter Mezei"} "."
                        " The source code is licensed under " a href="https://opensource.org/licenses/GPL-2.0" target="_blank" {"github.com/mezeipetister/bit"} "."
                        br;
                        " This project is available under " a href="https://github.com/mezeipetister/bit" target="_blank" { "github.com/mezeipetister/bit" } "."
                    }
                    p {
                        a href="https://bulma.io" {
                            img src="https://bulma.io/images/made-with-bulma.png" alt="Made with Bulma" width="128" height="24" /
                        }
                    }
                }
            }
        }
    }
}
