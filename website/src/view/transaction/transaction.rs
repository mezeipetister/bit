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

use crate::component::*;
use crate::view::View;
use chrono::prelude::*;
use core_lib::Transaction;
use maud::{html, Markup};
use num_format::{Locale, ToFormattedString};
use storaget::*;

pub struct ViewTransaction<'a, T>
where
    T: Transaction,
{
    transactions: &'a [DataObject<T>],
}

impl<'a, T> ViewTransaction<'a, T>
where
    T: Transaction,
{
    pub fn new(transactions: &'a [DataObject<T>]) -> Self {
        ViewTransaction { transactions }
    }
}

impl<'a, T> View for ViewTransaction<'a, T>
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
                                h2.title.is-spaced {"Transactions"}
                            }
                        }
                        .level-right {
                            .level-item {
                                a.button href="/transactions/new" accesskey="n" {"+"}
                            }
                        }
                    }
                }
                .container {
                    table.table.is-striped."is-size-7" {
                        thead {
                            tr {
                                th {"#"}
                                th {"Subject"}
                                th {"D/C"}
                                th {"Amount"}
                                th {"Date settlement"}
                                th {"Date created"}
                                th {"Created by"}
                            }
                        }
                        tbody {
                            @for transaction in self.transactions {
                                tr {
                                    td {
                                        (transaction.get(|a| a.get_id().to_owned()))
                                        // a href=(format!("/accounts/{}", transaction.get(|a| a.get_id().to_owned())))
                                        //  {(transaction.get(|a| a.get_id().to_owned()))}
                                    }
                                    td {(transaction.get(|a| a.get_subject()))}
                                    td {
                                        (transaction.get(|a| a.get_debit_credit().0))
                                        "/"
                                        (transaction.get(|a| a.get_debit_credit().1))
                                    }
                                    td {(format!("HUF {}", transaction.get(|a| a.get_amount().to_formatted_string(&Locale::hu))))}
                                    td {(transaction.get(|a| format!("{}-{}-{}",a.get_date_settlement().year(),
                                                                                a.get_date_settlement().month(),
                                                                                a.get_date_settlement().day())))}
                                    td {(transaction.get(|a| format!("{}-{}-{}",a.get_date_created().year(),
                                                                                a.get_date_created().month(),
                                                                                a.get_date_created().day())))}
                                    td {(transaction.get(|a| a.get_created_by()))}
                                }
                            }
                        }
                    }
                    // Insert pagination component
                    (Pagination::new(vec![1,2,3,4], 1,2).render())
                }
            }
        }
    }
}
