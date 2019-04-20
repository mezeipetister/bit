/**
 * BIT
 * Copyright (C) 2019 Peter Mezei <mezeipetister@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>
 */
use chrono::prelude::*;

/// Event, represents a business event.
/// We use double entry book keeping, so we need:
///
/// ## How it works:
///
/// - debit		=>	Debit account ID
/// - credit		=>	Credit account ID
/// - value		=>	U32, positive integer, business event transaction value
/// - Permormance_date	=>	Performance date, date when we count on this event
#[derive(Debug, PartialEq, Clone)]
pub struct Event {
    debit: String, // TODO: Consider using u32, but then how to manage accounts starts with 0, such as 011?
    credit: String, // TODO: Same as above.
    value: u32,
    performance_date: chrono::NaiveDate,
}

/// New event
/// Returns a new event
pub fn new_event(
    debit: &str,
    credit: &str,
    value: u32,
    performance_date: chrono::NaiveDate,
) -> Event {
    Event {
        debit: debit.to_string(),
        credit: credit.to_string(),
        value,
        performance_date,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_event() {
        // This should be ok!
        assert_eq!(
            new_event("1", "2", 3, chrono::NaiveDate::from_ymd(2019, 3, 20)).value,
            3
        );
    }
}
