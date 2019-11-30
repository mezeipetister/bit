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
use core_lib::Transaction;
use maud::{html, Markup};
use storaget::*;

pub struct ViewDashboard<T>
where
    T: Transaction,
{
    transactions: Vec<DataObject<T>>,
}

impl<T> ViewDashboard<T>
where
    T: Transaction,
{
    pub fn new(transactions: &Storage<T>) -> Self {
        let mut acc = transactions.into_data_objects();
        acc.sort_by_key(|a| a.get(|a| a.get_id().to_owned()));
        ViewDashboard { transactions: acc }
    }
}

impl<T> View for ViewDashboard<T>
where
    T: Transaction,
{
    fn render(&self) -> Markup {
        html! {
            section.section {
                .container.content {
                    .level.is-mobile {
                        .level-left {
                            .level-item {
                                h2.title.is-spaced {"Dashboard"}
                            }
                        }
                        // .level-right {
                        //     .level-item {
                        //         a.button href="/transactions/new" accesskey="n" {"+"}
                        //     }
                        // }
                    }
                }
                .container {
                    "Total transaction number: "
                    (self.transactions.len())
                }
                // .container {
                //     table.table.is-striped {
                //         thead {
                //             tr {
                //                 th {"#"}
                //                 th {"Name"}
                //                 th {"Description"}
                //                 th {"Items"}
                //             }
                //         }
                //         tbody {
                //             @for account in &self.accounts {
                //                 tr {
                //                     td {
                //                         a href=(format!("/accounts/{}", account.get(|a| a.get_id().to_owned())))
                //                          {(account.get(|a| a.get_id().to_owned()))}
                //                     }
                //                     td {(account.get(|a| a.get_name().to_owned()))}
                //                     td {(account.get(|a| a.get_description().to_owned()))}
                //                     td {"-"}
                //                 }
                //             }
                //         }
                //     }
                // }
            }
        }
    }
}
