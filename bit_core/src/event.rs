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
extern crate chrono;
use chrono::NaiveDate;

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
    performance_date: NaiveDate,
}

/// New event
/// Returns a new event
///
/// # Example
///
/// ```rust
/// extern crate chrono;  
/// use chrono::NaiveDate;
///
/// # use bit_core::event::*;
///
/// // Date helper                                    
/// let date = |y, m, d| NaiveDate::from_ymd(y, m, d);
///
/// new_event("1", "2", 3, date(2019,3,11));
/// ```
pub fn new_event(debit: &str, credit: &str, value: u32, performance_date: NaiveDate) -> Event {
    Event {
        debit: debit.trim().to_string(),
        credit: credit.trim().to_string(),
        value,
        performance_date,
    }
}

/// Add event
///
/// # Example
///
/// ```rust
/// extern crate chrono;  
/// use chrono::NaiveDate;
///
/// # use bit_core::event::*;
///
/// // Date helper                                    
/// let date = |y, m, d| NaiveDate::from_ymd(y, m, d);
///
/// let mut events: Vec<Event> = Vec::new();
/// add_event(&mut events, new_event("1", "2", 3, date(2019,3,11)));
/// ```
pub fn add_event(events: &mut Vec<Event>, event_to_add: Event) {
    events.push(event_to_add);
}

/// Get ledger by date
///
/// # Example
///
/// ```rust
/// extern crate chrono;
/// use chrono::NaiveDate;
///
/// # use bit_core::event::*;
///
/// // Events holder
/// let mut events: Vec<Event> = Vec::new();
/// // Date helper
/// let date = |y, m, d| NaiveDate::from_ymd(y, m, d);
///
/// add_event(&mut events, new_event("1", "2", 3, date(2019, 3, 11)));
/// get_ledger_by_account_id_and_by_date("1", &events, date(2019, 3, 11)); // This should be 3
/// ```
pub fn get_ledger_by_account_id_and_by_date(
    account_id: &str,
    events: &[Event],
    date: NaiveDate,
) -> i32 {
    let sum_debit: u32 = events
        .iter()
        .filter(|event| event.debit == account_id)
        .filter(|event| event.performance_date <= date)
        .map(|event| event.value)
        .sum::<u32>();
    let sum_credit: u32 = events
        .iter()
        .filter(|event| event.credit == account_id)
        .filter(|event| event.performance_date <= date)
        .map(|event| event.value)
        .sum::<u32>();
    (sum_debit as i32 - sum_credit as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_event() {
        // Date helper
        let date = |y, m, d| NaiveDate::from_ymd(y, m, d);

        // This should be ok!
        assert_eq!(new_event("1", "2", 3, date(2019, 3, 20)).value, 3);

        // Test trim
        let event = new_event("   1  ", "      2    ", 3, date(2019, 3, 20));
        assert_eq!(event.debit, "1"); // Test debit
        assert_eq!(event.credit, "2"); // Test credit
    }

    #[test]
    fn test_add_event() {
        // Events holder
        let mut events: Vec<Event> = Vec::new();

        // Date helper
        let date = |y, m, d| NaiveDate::from_ymd(y, m, d);

        add_event(&mut events, new_event("1", "2", 3, date(2019, 3, 11)));
        add_event(&mut events, new_event("4", "5", 6, date(2019, 3, 11)));
        add_event(&mut events, new_event("7", "8", 9, date(2019, 3, 11)));
        add_event(&mut events, new_event("10", "11", 12, date(2019, 3, 12)));
        add_event(&mut events, new_event("13", "14", 15, date(2019, 3, 12)));
        add_event(&mut events, new_event("16", "17", 18, date(2019, 3, 12)));

        assert_eq!(events.len(), 6); // This should be true
        assert_eq!(events[3].credit, 11.to_string()); // This should be true
    }

    #[test]
    fn test_get_ledger_by_date() {
        // Events holder
        let mut events: Vec<Event> = Vec::new();

        // Date helper
        let date = |y, m, d| NaiveDate::from_ymd(y, m, d);

        add_event(&mut events, new_event("1", "2", 3, date(2019, 3, 11)));
        add_event(&mut events, new_event("4", "5", 6, date(2019, 3, 11)));
        add_event(&mut events, new_event("7", "8", 9, date(2019, 3, 11)));
        add_event(&mut events, new_event("10", "11", 12, date(2019, 3, 12)));
        add_event(&mut events, new_event("13", "14", 15, date(2019, 3, 12)));
        add_event(&mut events, new_event("16", "17", 18, date(2019, 3, 12)));
        add_event(&mut events, new_event("2", "1", 2, date(2019, 3, 13)));

        // This should be ok
        assert_eq!(
            get_ledger_by_account_id_and_by_date("1", &events, date(2019, 3, 11)),
            3
        );

        // This should be ok
        assert_eq!(
            get_ledger_by_account_id_and_by_date("1", &events, date(2019, 3, 10)),
            0
        );

        // This should be ok
        assert_eq!(
            get_ledger_by_account_id_and_by_date("1", &events, date(2019, 3, 13)),
            1
        );
    }
}
