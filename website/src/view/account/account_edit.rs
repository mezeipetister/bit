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
use core_lib::Account;
use maud::{html, Markup};
use storaget::*;

pub struct ViewAccountEdit<T>
where
    T: Account,
{
    account: DataObject<T>,
}

impl<T> ViewAccountEdit<T>
where
    T: Account,
{
    pub fn new(account: DataObject<T>) -> Self {
        ViewAccountEdit { account }
    }
}

impl<T> View for ViewAccountEdit<T>
where
    T: Account,
{
    fn render(&self) -> Markup {
        html! {
            section.section {
                .container.content {
                    h2.title {(self.account.get(|a| a.get_name().to_owned()))}
                    form method="POST" action=(format!("/accounts/{}", self.account.get(|a|a.get_id().to_owned()))) {
                        .field {
                            label.label {"Account ID"}
                            .control {
                                input.input type="text" name="account_id" placeholder="e.g.: 161" autofocus? value=(self.account.get(|a| a.get_id().to_owned())) disabled?;
                            }
                        }
                        .field {
                            label.label {"Account name"}
                            .control {
                                input.input type="text" name="account_name" placeholder="e.g.: Investment" value=(self.account.get(|a| a.get_name().to_owned()));
                            }
                        }
                        .field {
                            label.label {"Account description"}
                            .control {
                                input.input type="text" name="account_description" placeholder="e.g.: Hello world" value=(self.account.get(|a| a.get_description().to_owned()));
                            }
                        }
                        .field {
                            label.label {"Is working?"}
                            .control {
                                @if self.account.get(|a| a.is_working()) {
                                    input type="checkbox" name="is_working" checked?;
                                } @else {
                                    input type="checkbox" name="is_working";
                                }
                            }
                        }
                        .field {
                            label.label {"Is inverse?"}
                            .control {
                                @if self.account.get(|a| a.is_inverse()) {
                                    input type="checkbox" name="is_inverse" checked?;
                                } @else {
                                    input type="checkbox" name="is_inverse";
                                }
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
