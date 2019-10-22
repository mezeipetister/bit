use crate::view::View;
use maud::{html, Markup};

pub struct ViewLogin {}

impl ViewLogin {
    pub fn new() -> Self {
        ViewLogin {}
    }
}

impl View for ViewLogin {
    fn render(&self) -> Markup {
        html! {
            section.section {
                .container {
                    p.has-text-centered.title.is-spaced {"Login"}
                    p.subtitle.has-text-centered."is-size-6" {
                        "BIT online financial"
                        br;
                        "and project information system"
                    }
                    form action="/login" method="POST" {
                        .column."is-6-mobile"."is-offset-3-mobile"."is-4-desktop"."is-offset-4-desktop" {
                            .field {
                                label.label { "Username" }
                                .control {
                                    input.input type="text" name="username" placeholder="e.g. John Smith" autofocus?;
                                }
                            }
                            .field {
                                label.label { "Password" }
                                .control {
                                    input.input type="password" name="password" placeholder="strong password";
                                }
                            }
                            .field {
                                .control {
                                    button.button.is-info.is-outlined type="submit" href="/login" { "Login" }
                                    a.button href="/login/reset_password" { "Forget password" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn render_error(&self) -> Markup {
        html! {
            section.section {
                .container {
                    p.has-text-centered.title.is-spaced { "Login failed" }
                    p.subtitle.has-text-centered."is-size-6" {
                        "Oooops!"
                        br;
                        "Wrong username or password."
                    }
                    .columns.is-mobile {
                        .column."is-6-mobile"."is-offset-3-mobile"."is-4-desktop"."is-offset-4-desktop" {
                            .field {
                                .control.has-text-centered {
                                    .button-group {
                                        form action="/login" method="GET" {
                                            button.button autofocus? {
                                                "Try again"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
