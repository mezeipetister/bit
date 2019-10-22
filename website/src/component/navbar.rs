use maud::{html, Markup};

pub struct Navbar {
    brand: &'static str,
}

impl Navbar {
    pub fn default() -> Markup {
        html! {
            nav.navbar.is-light role="navigation" aria-label="main navigation" {
                .navbar-brand {
                    a.navbar-item href="/" {
                        span.title {"BIT"}
                    }
                    a.navbar-burger.burger role="button" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample" {
                        span aria-hidden="true" {}
                        span aria-hidden="true" {}
                        span aria-hidden="true" {}
                    }
                }
                #navbarBasicExample.navbar-menu {
                    .navbar-start {
                        a.navbar-item { "Repositories" }
                        a.navbar-item { "Documentation" }
                    }
                    .navbar-end {
                        .navbar-item {
                            .buttons {
                                a.button.is-light {
                                    span.icon.is-small {
                                        i.fa.fa-user-cog {}
                                    }
                                    span { "Settings" }
                                }
                                a.button.is-light href="/logout" accesskey="i" {
                                    span.icon.is-small {
                                        i.fa.fa-sign-out-alt {}
                                    }
                                    span { "Log out" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
