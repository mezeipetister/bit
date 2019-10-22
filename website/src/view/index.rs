use crate::view::View;
use maud::{html, Markup};

pub struct ViewIndex {}

impl ViewIndex {
    pub fn new() -> Self {
        ViewIndex {}
    }
}

impl View for ViewIndex {
    fn render(&self) -> Markup {
        html! {
            section.section {
                .container {
                    p."title"."is-3" { "Hello World "}
                    p."subtitle"."is-4" { "Your name is: EMPTY" }
                    p.content {
                        "lorem ipsum dolorem set ami"
                    }
                }
            }
        }
    }
}
