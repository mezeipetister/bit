use crate::component::*;
use maud::{html, Markup, PreEscaped, DOCTYPE};

pub struct Layout<'a> {
    pub title: Option<&'a str>,
    pub meta_keywords: Option<Vec<&'a str>>,
    pub meta_description: Option<&'a str>,
    pub meta_author: Option<&'a str>,
    pub meta_language: Option<&'a str>,
    pub meta_robots: Option<&'a str>,
    pub meta_designer: Option<&'a str>,
    pub meta_publisher: Option<&'a str>,
    pub navbar: Option<Markup>,
    pub footer: Option<Markup>,
}

impl<'a> Layout<'a> {
    pub fn new() -> Self {
        Layout {
            title: None,
            meta_keywords: None,
            meta_description: None,
            meta_author: None,
            meta_language: None,
            meta_robots: None,
            meta_designer: None,
            meta_publisher: None,
            navbar: Some(Navbar::default()),
            footer: Some(Footer::default()),
        }
    }
    pub fn set_title(&mut self, title: &'a str) -> &'a mut Layout {
        self.title = Some(title);
        self
    }
    pub fn render(&self, body: Markup) -> Markup {
        html! {
            (DOCTYPE)
            html {
                head {
                    meta charset="utf-8" /
                    meta name="viewport" content="width=device-width, initial-scale=1" /
                    title {(self.title.unwrap_or("TITLE"))}
                    link rel="stylesheet" type="text/css" href="/static/style.css" /
                    link rel="icon" type="image/x-icon" href="data:image/x-icon;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQEAYAAABPYyMiAAAABmJLR0T///////8JWPfcAAAACXBIWXMAAABIAAAASABGyWs+AAAAF0lEQVRIx2NgGAWjYBSMglEwCkbBSAcACBAAAeaR9cIAAAAASUVORK5CYII=" /
                }
                body {
                    (&self.navbar.as_ref().unwrap_or(&html!{}))
                    (body)
                    (&self.footer.as_ref().unwrap_or(&html!{}))
                    script defer? src="/static/fa.js" {}
                    script defer? src="/static/script.js" {}
                }
            }
        }
    }
}
