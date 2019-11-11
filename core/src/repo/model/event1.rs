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

use crate::balance::*;
use crate::prelude::*;
use chrono::prelude::*;

pub struct Event1 {
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
