use crate::view::View;
use maud::{html, Markup};

pub struct _ {}

impl _ {
    pub fn new() -> Self {
        _ {}
    }
}

impl View for _ {
    fn render(&self) -> Markup {
        html! {
            
        }
    }
}
