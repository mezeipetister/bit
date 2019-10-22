use maud::{html, Markup};

pub struct PageIndex<'a> {
    text: &'a str,
}

impl<'a> PageIndex<'a> {
    pub fn render(name: &'a str) -> Markup {
        html! {
            section.section {
                .container {
                    p."title"."is-3" { "Hello World "}
                    p."subtitle"."is-4" { "Your name is: " (name) }
                    p.content {
                        "lorem ipsum dolorem set ami"
                    }
                }
            }
        }
    }
}
