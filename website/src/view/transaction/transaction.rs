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

pub struct ViewTransaction<T>
where
    T: Transaction,
{
    transactions: Vec<DataObject<T>>,
}

impl<T> ViewTransaction<T>
where
    T: Transaction,
{
    pub fn new(transactions: &Storage<T>) -> Self {
        ViewTransaction {
            transactions: transactions.into_data_objects(),
        }
    }
}

impl<T> View for ViewTransaction<T>
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
                    table.table.is-striped {
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
                            @for transaction in &self.transactions {
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
                                    td {(format!("{}-{}-{}",    transaction.get(|a| a.get_date_settlement().year()),
                                                                transaction.get(|a| a.get_date_settlement().month()),
                                                                transaction.get(|a| a.get_date_settlement().day())))}
                                    td {(format!("{}-{}-{}",    transaction.get(|a| a.get_date_created().year()),
                                                                transaction.get(|a| a.get_date_created().month()),
                                                                transaction.get(|a| a.get_date_created().day())))}
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
