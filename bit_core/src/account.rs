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

/// Account, represents an ledger account.
/// We use double entry book keeping, so we need:
///
/// ## How it works:
///
/// - id	=>	Account ID
/// - name	=>	Account name
#[derive(Debug, PartialEq, Clone)]
pub struct Account {
    id: String,
    name: String,
}

/// New Account
/// Returns a new account.
pub fn new_account(id: &str, name: &str) -> Account {
    Account {
        id: id.to_string(),
        name: name.to_string(),
    }
}

/// Validate account by ID
/// Once we have a given account ID,
/// we can check wheter it is valid or not.
///
/// # Example
///
/// ```rust
/// # use bit_core::account::*;
///
/// let mut accounts:Vec<Account> = Vec::new();  
/// accounts.push(new_account("1", "A"));  
/// accounts.push(new_account("2", "B"));  
/// accounts.push(new_account("3", "C"));  
/// accounts.push(new_account("4", "D"));  
///
/// let it_contains = is_valid_account(&accounts, "3"); // returns true
/// ```
pub fn is_valid_account(accounts: &[Account], account_id_to_check: &str) -> bool {
    accounts
        .iter()
        .any(|account| account.id == account_id_to_check)
}

/// Get account by id
///
/// # Example
///
/// ```rust
/// # use bit_core::account::*;
///
/// // TODO: Continue
/// ```
pub fn get_account_by_id<'a>(
    accounts: &'a Vec<Account>,
    account_id_to_get: &str,
) -> Result<&'a Account, &'a str> {
    if is_valid_account(accounts, account_id_to_get) {
        match accounts
            .iter()
            .find(|account| account.id == account_id_to_get.to_string())
        {
            Some(account) => return Ok(account),
            None => return Err("Not found"),
        }
    }
    Err("Not found")
}

/// Add account to an account vector
///
/// # Example
///
/// ```rust
/// # use bit_core::account::*;
///
/// let mut accounts: Vec<Account> = Vec::new();
/// add_account(&mut accounts, new_account("1", "A")).unwrap();
/// add_account(&mut accounts, new_account("2", "B")).unwrap();
/// add_account(&mut accounts, new_account("3", "C")).unwrap();
/// ```
pub fn add_account(accounts: &mut Vec<Account>, account: Account) -> Result<(), &str> {
    if !is_valid_account(accounts, &account.id) {
        accounts.push(account);
        return Ok(());
    }
    Err("Account already exists")
}

/// Check if a given account ID is leaf or not
///
/// # Example
///
/// ```rust
/// # use bit_core::account::*;
///
/// let mut accounts: Vec<Account> = Vec::new();  
/// accounts.push(new_account("1", "A"));  
/// accounts.push(new_account("11", "B"));  
/// accounts.push(new_account("111", "C"));  
/// accounts.push(new_account("1111", "D"));  
///
/// check_account_is_leaf(&accounts, "2"); // => false  
/// check_account_is_leaf(&accounts, "1"); // => false  
/// check_account_is_leaf(&accounts, "11"); // => false  
/// check_account_is_leaf(&accounts, "1111"); // => true  
/// ```
pub fn check_account_is_leaf(accounts: &[Account], account_id_to_check: &str) -> bool {
    accounts
        .iter()
        .filter(|account| {
            account
                .id
                .chars()
                .take(account_id_to_check.len())
                .collect::<Vec<char>>()
                == account_id_to_check.chars().collect::<Vec<char>>()
        })
        .count()
        == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_account() {
        let mut accounts: Vec<Account> = Vec::new();
        accounts.push(new_account("1", "A"));
        accounts.push(new_account("2", "B"));
        accounts.push(new_account("3", "C"));
        accounts.push(new_account("4", "D"));

        // This should be true
        assert!(is_valid_account(&accounts, "3"));
        // This should be false
        // But as we negate, it should be ok! !true => false, which means true
        assert!(!is_valid_account(&accounts, "5"));
    }

    #[test]
    fn test_add_account() {
        let mut accounts: Vec<Account> = Vec::new();
        // These should be ok
        assert!(add_account(&mut accounts, new_account("1", "A")).is_ok());
        assert!(add_account(&mut accounts, new_account("2", "B")).is_ok());
        assert!(add_account(&mut accounts, new_account("3", "C")).is_ok());
        assert!(add_account(&mut accounts, new_account("4", "D")).is_ok());

        // This should returns an error
        assert!(add_account(&mut accounts, new_account("3", "C")).is_err());
    }

    #[test]
    fn test_check_account_is_leaf() {
        let mut accounts: Vec<Account> = Vec::new();
        // These should be ok
        assert!(add_account(&mut accounts, new_account("1", "A")).is_ok());
        assert!(add_account(&mut accounts, new_account("11", "B")).is_ok());
        assert!(add_account(&mut accounts, new_account("111", "C")).is_ok());
        assert!(add_account(&mut accounts, new_account("1111", "D")).is_ok());

        assert!(!check_account_is_leaf(&accounts, "2")); // This should be false
        assert!(!check_account_is_leaf(&accounts, "1")); // This should be false
        assert!(!check_account_is_leaf(&accounts, "11")); // This should be false
        assert!(!check_account_is_leaf(&accounts, "111")); // This should be false
        assert!(check_account_is_leaf(&accounts, "1111")); // This should be true
    }

    #[test]
    fn test_get_account_by_id() {
        let mut accounts: Vec<Account> = Vec::new();
        // These should be ok
        assert!(add_account(&mut accounts, new_account("1", "A")).is_ok());
        assert!(add_account(&mut accounts, new_account("2", "B")).is_ok());
        assert!(add_account(&mut accounts, new_account("3", "C")).is_ok());
        assert!(add_account(&mut accounts, new_account("4", "D")).is_ok());

        assert_eq!(get_account_by_id(&accounts, "3").unwrap().name, "C"); // This should be true
        assert!(get_account_by_id(&accounts, "5").is_err()); // This should be false!
    }
}
