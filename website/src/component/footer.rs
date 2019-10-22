use crate::component::Component;
use maud::{html, Markup};

pub struct Footer {}

impl Component for Footer {
    fn default() -> Markup {
        html! {
            footer.footer {
                .content.has-text-centered {
                    p.small.has-text-weight-light {
                        strong {"BIT"} " by" a href="https://github.com/mezeipetister" target="_blank" accesskey="r" {"Peter Mezei"} "."
                        "The source code is licensed under " a href="https://opensource.org/licenses/GPL-2.0" target="_blank" {"github.com/mezeipetister/bit"} "."
                        br;
                        "This project is available under " a href="https://github.com/mezeipetister/bit" target="_blank" { "github.com/mezeipetister/bit" } "."
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
