// Copyright (C) 2020 Peter Mezei
//
// This file is part of GNStore.
//
// GNStore is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// GNStore is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with GNStore.  If not, see <http://www.gnu.org/licenses/>.

pub trait CashRegister {
    /// Add money to a cash_register,
    /// returns the latest balance.
    fn add_money(
        &mut self,
        // Who made the transaction
        // not customer
        userid: String,
        // Now its only HUF & integer
        money: u32,
        // If transaction
        cartId: Option<usize>,
        comment: String,
    ) -> u32;
}
