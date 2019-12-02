// Copyright (C) 2019 Peter Mezei
//
// This file is part of Project A.
//
// Project A is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// Project A is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Project A.  If not, see <http://www.gnu.org/licenses/>.

use maud::Markup;

pub mod footer;
pub mod navbar;
pub mod notification;
pub mod pagination;
pub mod tabbar;

pub use footer::Footer;
pub use navbar::Navbar;
pub use notification::Notification;
pub use pagination::Pagination;
pub use tabbar::TabBar;

pub trait Component {
    fn render(&self) -> Markup;
}
