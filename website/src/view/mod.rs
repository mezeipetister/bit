use maud::Markup;

pub mod index;
pub mod login;
pub mod not_found_404;

pub use index::ViewIndex;
pub use login::ViewLogin;
pub use not_found_404::View404;

pub trait View {
    fn render(&self) -> Markup;
    // Default implementation for success
    fn render_success(&self) -> Markup {
        self.render()
    }
    // Default implementation for error
    fn render_error(&self) -> Markup {
        self.render()
    }
}
