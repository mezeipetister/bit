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

use chrono::prelude::*;

pub struct Transaction {
    // Transaction ID
    id: String,
    debit: String,
    credit: String,
    amount: String,
    date_created: DateTime<Utc>,
    date_settlement: Date<Utc>,
    // date_posting: Date<Utc>,
    // Duedate should be a part of event details
    // duedate: Date<Utc>,
    created_by: String,
    description: String,
    event_id: String,
    // commit_id: String,
    // If commit accepted, is_accepted field is true.
    // Only true when it is a part of the ledger.
    // At this point, any event transaction will be a part
    // of ledger immediately.
    // is_ledger_memeber: bool,
}

pub struct Event {
    id: String,
    title: String,
    description: String,
    reference: String,
    created_by: String,
    date_created: DateTime<Utc>,
    date_settlement: Date<Utc>,
    date_posting: Date<Utc>,
    duedate: Date<Utc>,
}

pub struct Account {
    id: String,
    name: String,
    description: String,
    created_by: String,
    date_created: DateTime<Utc>,
    is_inverse: bool,
    // Can we use it as an active ID to account?
    // True yes, false no
    is_working: bool,
    // Can we use transaction match
    is_matching: bool,
    is_balance_zero: bool,
}
