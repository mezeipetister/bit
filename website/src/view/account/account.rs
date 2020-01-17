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

use crate::view::View;
use core_lib::{Account, Transaction};
// use ifeq::*;
use maud::{html, Markup};
use storaget::*;

pub struct ViewAccount<'a, T, U>
where
    T: Account,
    U: Transaction,
{
    accounts: Vec<DataObject<T>>,
    transactions: &'a Storage<U>,
}

impl<'a, T, U> ViewAccount<'a, T, U>
where
    T: Account,
    U: Transaction,
{
    pub fn new(accounts: &Storage<T>, transactions: &'a Storage<U>) -> Self {
        let mut acc = accounts.into_data_objects();
        acc.sort_by_key(|a| a.get(|a| a.get_id().to_owned()));
        ViewAccount {
            accounts: acc,
            transactions,
        }
    }
}

impl<'a, T, U> View for ViewAccount<'a, T, U>
where
    T: Account,
    U: Transaction,
{
    fn render(&self) -> Markup {
        html! {
            section.section {
                .container.content {
                    .level.is-mobile {
                        .level-left {
                            .level-item {
                                h2.title.is-spaced {"Accounts"}
                            }
                        }
                        .level-right {
                            .level-item {
                                a.button href="/accounts/new" accesskey="n" {"+"}
                            }
                        }
                    }
                }
                .container {
                    table.table.is-striped."is-size-7" {
                        thead {
                            tr {
                                th {
                                    span.icon.is-small.has-text-grey {
                                        i.fa.fa-exchange-alt aria-hidden="true" title="Is inverse" {}
                                    }
                                }
                                th {
                                    span.icon.is-small.has-text-success {
                                        i.far.fa-check-circle aria-hidden="true" title="Is working" {}
                                    }
                                }
                                th {"#"}
                                th {"Name"}
                                th {"Description"}
                                th {"Items"}
                            }
                        }
                        tbody {
                            @for account in &self.accounts {
                                @let account = account.get_data_ref();
                                tr {
                                    td {
                                        @if account.is_inverse() {
                                            span.icon.is-small.has-text-grey {
                                                i.fa.fa-exchange-alt aria-hidden="true" title="Is inverse" {}
                                            }
                                        }
                                    }
                                    td {
                                        @if account.is_working() {
                                            span.icon.is-small.has-text-success {
                                                i.far.fa-check-circle aria-hidden="true" title="Is working" {}
                                            }
                                        }
                                    }
                                    td {
                                        a href=(format!("/accounts/{}", account.get_id().to_owned()))
                                         {(account.get_id().to_owned())}
                                    }
                                    td {(account.get_name().to_owned())}
                                    td {(account.get_description().to_owned())}
                                    td {(&self.transactions.into_iter().filter(|o|o.get(|a| {
                                            a.get_credit() == account.get_id().to_owned()
                                            || a.get_debit() == account.get_id().to_owned()
                                        })).collect::<Vec<_>>().len()
                                    )}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
