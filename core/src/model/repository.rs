// Copyright (C) 2020 peter
//
// This file is part of BIT.
//
// BIT is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// BIT is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with BIT.  If not, see <http://www.gnu.org/licenses/>.

pub struct Repository {
    id: String,
    accounts: Vec<Account>,
    transactions: Vec<Transaction>,
    assets: Vec<Asset>,
    projects: Vec<Project>,
}

pub struct Account {}
pub struct Transaction {}
pub struct Asset {}
pub struct Project {}
