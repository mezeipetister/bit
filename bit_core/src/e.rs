/* BIT
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

// use chrono::prelude::*;

/// Event, represents a business event.
/// We use double entry book keeping, so we need:
///
/// ## How it works:
///
/// - debit		=>	Debit account ID
/// - credit		=>	Credit account ID
/// - value		=>	U32, positive integer, business event transaction value
/// - Permormance_date	=>	Performance date, date when we count on this event
///
/// TODO: Check/Validate english terms, such as performance_date
#[derive(Debug, PartialEq, Clone)]
pub struct Event {
    debit: String, // TODO: Consider using u32, but then how to manage accounts starts with 0, such as 011?
    credit: String, // TODO: Same as above.
    value: u32,
    performance_date: chrono::NaiveDate,
}

/// New Event
/// Returns a new event.
pub fn new_event(
    debit: String,
    credit: String,
    value: u32,
    performance_date: chrono::NaiveDate,
) -> Event {
    Event {
        debit: debit.trim().to_string(),
        credit: credit.trim().to_string(),
        value,
        performance_date,
    }
}

// pub fn get_all_account_balance_by_month(
//     el: &EventLog,
//     date: chrono::NaiveDate,
// ) -> HashMap<String, i32> {
//     let mut result: HashMap<String, i32> = HashMap::new();

//     for account in self.get_account_list() {
//         let d_sum: u32 = self
//             .events
//             .iter()
//             .filter(|e| e.debit == account)
//             .filter(|e| e.performance_date <= date)
//             .map(|e| e.value)
//             .sum();
//         let c_sum: u32 = self
//             .events
//             .iter()
//             .filter(|e| e.credit == account)
//             .filter(|e| e.performance_date <= date)
//             .map(|e| e.value)
//             .sum();

//         result.insert(account, d_sum as i32 - c_sum as i32);
//     }

//     result
// }
