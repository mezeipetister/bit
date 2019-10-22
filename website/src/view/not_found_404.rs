use maud::{html, Markup};

pub struct View404 {}

impl View404 {
    pub fn render(path: &str) -> Markup {
        html! {
            section.hero.is-fullheight {
                .hero-body {
                    .container.has-text-centered {
                        p.content {
                            span.icon {
                                i.fab.fa-earlybirds."fa-10x" {}
                            }
                        }
                        h1.title { "Ooo. Page not found." }
                        p.subtitle { "Path: " (path)}
                    }
                }
            }
        }
    }
}
