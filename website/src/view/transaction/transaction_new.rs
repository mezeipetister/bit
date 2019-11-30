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
// use core_lib::Account;
use chrono::prelude::*;
use maud::{html, Markup};
// use storaget::*;

pub struct ViewTransactionNew {}

impl ViewTransactionNew {
    pub fn new() -> Self {
        ViewTransactionNew {}
    }
}

impl View for ViewTransactionNew {
    fn render(&self) -> Markup {
        let today = format!(
            "{}-{}-{}",
            Utc::today().year(),
            Utc::today().month(),
            Utc::today().day()
        );
        html! {
            section.section {
                .container.content {
                    h2.title {"New transaction"}
                    form method="POST" action="/transactions/new" {
                        .field {
                            label.label {"Subject"}
                            .control {
                                input.input type="text" name="transaction_subject" placeholder="e.g.: Example invoice" autofocus? required?;
                            }
                        }
                        .field {
                            label.label {"Debit"}
                            .control {
                                input.input type="text" name="transaction_debit" placeholder="e.g.: 161" required?;
                            }
                        }
                        .field {
                            label.label {"Credit"}
                            .control {
                                input.input type="text" name="transaction_credit" placeholder="e.g.: 3841" required?;
                            }
                        }
                        .field {
                            label.label {"Amount"}
                            .control {
                                input.input type="text" name="transaction_amount" placeholder="e.g.: 1000" required?;
                            }
                        }
                        .field {
                            label.label {"Settlement date"}
                            .control {
                                input.input type="date" name="transaction_date_settlement" placeholder="e.g.: 2020-01-01" required? value=(today);
                            }
                        }
                        .buttons {
                            button.button.is-primary.is-outlined type="submit" {"Save"}
                            a.button href="." {"Cancel"}
                        }
                    }
                }
            }
        }
    }
}
