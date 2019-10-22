use maud::Markup;

pub mod index;
pub mod not_found_404;

pub use index::ViewIndex;
pub use not_found_404::View404;

pub trait View {
    fn render(&self) -> Markup;
}
