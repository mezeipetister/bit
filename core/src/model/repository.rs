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

use crate::error::Error;
use crate::prelude::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use storaget::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Repository {
    /// Repository ID
    /// Automatically generated
    id: String,
    /// Repository name
    /// Created by user
    name: String,
    /// Sort description
    description: String,
    /// Account vector
    accounts: Vec<Account>,
    /// Transaction vector
    transactions: Vec<Transaction>,
    /// Asset vector
    assets: Vec<Asset>,
    /// Project vector
    projects: Vec<Project>,
    /// Created by userid
    created_by: String,
    /// Date created
    date_created: DateTime<Utc>,
    /// Is active
    /// Logical remove
    /// False means its removed
    is_active: bool,
}

impl Repository {
    pub fn new(name: String, description: String, created_by: String) -> Self {
        Repository {
            id: generate_id(5),
            name: name.trim().to_string(),
            description: description.trim().to_string(),
            accounts: Vec::new(),
            transactions: Vec::new(),
            assets: Vec::new(),
            projects: Vec::new(),
            created_by,
            date_created: Utc::now(),
            is_active: true,
        }
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn get_created_by(&self) -> &str {
        &self.created_by
    }
    pub fn get_date_created(&self) -> DateTime<Utc> {
        self.date_created
    }
    pub fn get_is_active(&self) -> bool {
        self.is_active
    }
    pub fn remove(&mut self) {
        self.is_active = false;
    }
    pub fn restore(&mut self) {
        self.is_active = true;
    }
    pub fn add_account(&mut self, account: Account) -> AppResult<()> {
        if let Some(_) = self.accounts.iter().position(|a| a.get_id() == account.id) {
            return Err(Error::BadRequest(
                "A megadott accoutn ID már létezik!".to_string(),
            ));
        }
        // Check if account id contains just numbers
        if !account.get_id().chars().all(char::is_numeric) {
            return Err(Error::BadRequest(
                "A számla száma csak számokat tartalmazhat!".to_string(),
            ));
        }
        self.accounts.push(account);
        self.accounts.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(())
    }
    pub fn get_accounts(&self) -> &Vec<Account> {
        &self.accounts
    }
    pub fn get_account_by_id(&self, id: String) -> AppResult<Account> {
        for account in &self.accounts {
            if account.get_id() == id {
                return Ok(account.clone());
            }
        }
        Err(Error::BadRequest(
            "A megadott ID-val account nem szerepel".to_string(),
        ))
    }
    pub fn is_valid_account(&self, id: &str) -> bool {
        for account in &self.accounts {
            if account.get_id() == id && account.get_is_working() && account.get_is_active() {
                return true;
            }
        }
        false
    }
    pub fn update_account(
        &mut self,
        account_id: String,
        name: String,
        description: String,
        is_working: bool,
        is_inverse: bool,
    ) -> AppResult<Account> {
        for account in &mut self.accounts {
            if account.id == account_id {
                account.name = name;
                account.description = description;
                account.is_working = is_working;
                account.is_inverse = is_inverse;
                return Ok(account.clone());
            }
        }
        Err(Error::BadRequest(
            "A számla azonosító nem található".to_string(),
        ))
    }
    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
    pub fn get_transaction_by_id(&self, id: usize) -> AppResult<Transaction> {
        for item in &self.transactions {
            if item.id == id {
                return Ok(item.clone());
            }
        }
        Err(Error::BadRequest(
            "A megadott tranzakció nem található".to_string(),
        ))
    }
    pub fn add_transaction(
        &mut self,
        subject: String,
        debit: String,
        credit: String,
        amount: i32,
        date_settlement: NaiveDate,
        created_by: String,
    ) -> AppResult<Transaction> {
        if !self.is_valid_account(&debit) {
            return Err(Error::BadRequest(
                "A megadott debit ID nem könyvelhető".to_string(),
            ));
        }
        if !self.is_valid_account(&credit) {
            return Err(Error::BadRequest(
                "A megadott credit ID nem könyvelhető".to_string(),
            ));
        }
        let transaction = Transaction::new(
            self.transactions.len(),
            subject,
            debit,
            credit,
            amount,
            date_settlement,
            created_by,
        );
        self.transactions.push(transaction.clone());
        self.transactions
            .sort_by(|a, b| a.date_settlement.cmp(&b.date_settlement));
        Ok(transaction)
    }
}

impl StorageObject for Repository {
    type ResultType = Repository;
    fn get_id(&self) -> &str {
        &self.id
    }
    fn try_from(from: &str) -> StorageResult<Self::ResultType> {
        match deserialize_object(from) {
            Ok(res) => Ok(res),
            Err(_) => Err(storaget::Error::DeserializeError(
                "document has wrong format".to_string(),
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    id: String,
    name: String,
    description: String,
    created_by: String,
    date_created: DateTime<Utc>,
    is_working: bool,
    is_inverse: bool,
    is_active: bool,
}

impl Account {
    pub fn new(
        id: String,
        name: String,
        description: String,
        created_by: String,
        is_working: bool,
        is_inverse: bool,
    ) -> Self {
        Account {
            id: id.trim().to_string(),
            name,
            description,
            created_by,
            date_created: Utc::now(),
            is_working,
            is_inverse,
            is_active: true,
        }
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn get_created_by(&self) -> &str {
        &self.created_by
    }
    pub fn get_date_created(&self) -> DateTime<Utc> {
        self.date_created
    }
    pub fn get_is_working(&self) -> bool {
        self.is_working
    }
    pub fn get_is_inverse(&self) -> bool {
        self.is_inverse
    }
    pub fn set_is_inverse(&mut self, is_inverse: bool) {
        self.is_inverse = is_inverse;
    }
    pub fn get_is_active(&self) -> bool {
        self.is_active
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub id: usize,
    pub subject: String,
    pub debit: String,
    pub credit: String,
    pub amount: i32,
    pub date_created: DateTime<Utc>,
    pub date_settlement: NaiveDate,
    pub created_by: String,
}

impl Transaction {
    pub fn new(
        id: usize,
        subject: String,
        debit: String,
        credit: String,
        amount: i32,
        date_settlement: NaiveDate,
        created_by: String,
    ) -> Self {
        Transaction {
            id,
            subject,
            debit,
            credit,
            amount,
            date_created: Utc::now(),
            date_settlement,
            created_by,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Asset {
    id: usize,
    name: String,
    description: String,
    tag: String,
    value: u32,
    date_activated: NaiveDate,
    depreciation_key: f32,
    residual_value: u32,
    date_created: DateTime<Utc>,
    created_by: String,
    is_active: bool,
}

impl Asset {
    pub fn new(
        id: usize,
        name: String,
        description: String,
        tag: String,
        value: u32,
        date_activated: NaiveDate,
        depreciation_key: f32,
        residual_value: u32,
        created_by: String,
    ) -> Self {
        Asset {
            id,
            name,
            description,
            tag,
            value,
            date_activated,
            depreciation_key,
            residual_value,
            date_created: Utc::now(),
            created_by,
            is_active: true,
        }
    }
    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn get_tag(&self) -> &str {
        &self.tag
    }
    pub fn set_tag(&mut self, tag: String) {
        self.tag = tag;
    }
    pub fn remove(&mut self) {
        self.is_active = false;
    }
    pub fn restore(&mut self) {
        self.is_active = true;
    }
    pub fn get_is_active(&self) -> bool {
        self.is_active
    }
    pub fn get_value(&self) -> u32 {
        self.value
    }
    pub fn get_depreciation_key(&self) -> f32 {
        self.depreciation_key
    }
    pub fn get_date_created(&self) -> DateTime<Utc> {
        self.date_created
    }
    pub fn get_created_by(&self) -> &str {
        &self.created_by
    }
    pub fn get_residual_value(&self) -> u32 {
        self.residual_value
    }
    pub fn get_date_activated(&self) -> NaiveDate {
        self.date_activated
    }
    pub fn depreciation_value(&self) -> u32 {
        self.value - self.residual_value
    }
    pub fn depreciation_daily_value(&self) -> u32 {
        (((self.value - self.residual_value) as f32 * self.depreciation_key) / 100.0 / 365.0)
            .round() as u32
    }
    pub fn depreciation_days(&self) -> u32 {
        ((self.value - self.residual_value) as f32 / self.depreciation_daily_value() as f32).ceil()
            as u32
    }
    pub fn depreciation_last_day(&self) -> NaiveDate {
        // Minus 1 as we count depreciation for the first day as well
        self.date_activated + chrono::Duration::days(self.depreciation_days() as i64 - 1)
    }
    pub fn depreciation_last_day_value(&self) -> u32 {
        self.depreciation_value() - (self.depreciation_days() - 1) * self.depreciation_daily_value()
    }
    pub fn depreciation_by_month(&self, year: i32, month: i32) -> u32 {
        let last_day = self.depreciation_last_day();
        let date = NaiveDate::from_ymd(year, month as u32, 1);
        let date_month_day_number = month_last_day(date).day();
        // If month is withing the current item depreciation interval
        if date >= (self.date_activated - chrono::Duration::days(self.date_activated.day() as i64))
            && date <= last_day
        {
            // if the month is the last month
            // and it might be not a full one
            if date.month() == last_day.month() && date.year() == last_day.year() {
                return (last_day.day() - 1) * self.depreciation_daily_value()
                    + self.depreciation_last_day_value();
            }
            // if the month if the first month
            // and it might be not a full one
            else if date.month() == self.date_activated.month()
                && date.year() == self.date_activated.year()
            {
                return (date_month_day_number - self.date_activated.day() + 1)
                    * self.depreciation_daily_value();
            } else {
                return date_month_day_number * self.depreciation_daily_value();
            }
        }
        0
    }
    /// return Vec<Date, Depreciation value, Cumulated depreciation>
    pub fn depreciation_monthly_vector(&self) -> Vec<(NaiveDate, u32, u32)> {
        let mut res: Vec<(NaiveDate, u32, u32)> = Vec::new();
        let mut date_next: NaiveDate = month_last_day(self.date_activated);
        for _ in 0.. {
            match self.depreciation_by_month(date_next.year(), date_next.month() as i32) {
                x if x != 0 => res.push((
                    date_next,
                    x,
                    match res.last() {
                        Some(t) => t.2 + x,
                        None => x,
                    },
                )),
                _ => break,
            }
            date_next = next_month_last_day(date_next);
        }
        res
    }
    pub fn depreciation_total_till_date(&self, date: NaiveDate) -> u32 {
        self.depreciation_monthly_vector()
            .into_iter()
            .filter(|i| i.0 <= date)
            .map(|i| i.1)
            .sum()
    }
}

fn month_last_day(date: NaiveDate) -> NaiveDate {
    let given_first_day = NaiveDate::from_ymd(date.year(), date.month(), 1);
    let next_month_some = given_first_day + chrono::Duration::days(32);
    NaiveDate::from_ymd(next_month_some.year(), next_month_some.month(), 1)
        - chrono::Duration::days(1)
}

fn next_month_last_day(date: NaiveDate) -> NaiveDate {
    month_last_day(month_last_day(date) + chrono::Duration::days(1))
}

impl Repository {
    pub fn add_asset(
        &mut self,
        name: String,
        description: String,
        tag: String,
        value: u32,
        date_activated: NaiveDate,
        deprecation_key: f32,
        residual_value: u32,
        created_by: String,
    ) {
        let new_asset = Asset::new(
            id: self.assets.len(),
            name,
            description,
            tag,
            value,
            date_activated,
            depreciation_key,
            residual_value,
            created_by,
        );
        self.assets.push(new_asset);
    }
    pub fn remove_asset_by_id(&mut self, id: usize) -> AppResult<()> {
        for asset in &mut self.assets {
            if asset.get_id() == id {
                asset.remove();
                return Ok(());
            }
        }
        Err(Error::BadRequest("Asset id not found".to_string()))
    }
    pub fn restore_asset_by_id(&mut self, id: usize) -> AppResult<()> {
        for asset in &mut self.assets {
            if asset.get_id() == id {
                asset.restore();
                return Ok(());
            }
        }
        Err(Error::BadRequest("Asset id not found".to_string()))
    }
    pub fn asset_get_by_id(&mut self, id: usize) -> AppResult<Asset> {
        for asset in &self.assets {
            if asset.get_id() == id {
                return Ok(asset.clone());
            }
        }
        Err(Error::BadRequest("Asset id not found".to_string()))
    }
    pub fn asset_update_by_id(&mut self, id: usize, name: String, description: String, tag: String) -> AppResult<()> {
        for asset in &self.assets {
            if asset.get_id() == id {
                asset.set_name(name);
                asset.set_description(description);
                asset.set_tag(tag);
                return Ok(());
            }
        }
        Err(Error::BadRequest("Asset id not found".to_string()))
    }
    pub fn asset_get_tags(&self) -> Vec<String> {
        let mut res = self.assets.iter().map(|i: &Asset|i.get_tag().to_string()).collect::<Vec<String>>();
        res.sort();
        res.dedup();
        res
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {}
