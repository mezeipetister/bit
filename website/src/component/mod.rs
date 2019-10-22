use maud::Markup;

pub mod footer;
pub mod navbar;
pub mod tabbar;

pub use footer::Footer;
pub use navbar::Navbar;
pub use tabbar::TabBar;

pub trait Component {
    fn default() -> Markup;
}