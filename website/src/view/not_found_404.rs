use crate::view::View;
use maud::{html, Markup};

pub struct View404<'a> {
    path: Option<&'a str>,
}

impl<'a> View404<'a> {
    pub fn new(path: &'a str) -> Self {
        View404 { path: Some(path) }
    }
}

impl<'a> View for View404<'a> {
    fn render(&self) -> Markup {
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
                        p.subtitle { "Path: " (self.path.unwrap_or("NONE"))}
                    }
                }
            }
        }
    }
}
