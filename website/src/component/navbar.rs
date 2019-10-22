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
            section.section {
                .container {
                    .tabs.is-centered.is-boxed {
                        ul {
                            li.is-active {
                                a href="/dashboard" {
                                    span.icon.is-small {
                                        i.fa.fa-tachometer-alt aria-hidden="true" {}
                                    }
                                    span { "Dashboard" }
                                }
                            }
                            li {
                                a href="/commits" {
                                    span.icon.is-small {
                                        i.fa.fa-history aria-hidden="true" {}
                                    }
                                    span { "Commits" }
                                }
                            }
                            li {
                                a href="/accounts" {
                                    span.icon.is-small {
                                        i.fa.fa-list-alt aria-hidden="true" {}
                                    }
                                    span { "Accounts" }
                                }
                            }
                            li {
                                a href="/projects" {
                                    span.icon.is-small {
                                        i.fa.fa-project-diagram aria-hidden="true" {}
                                    }
                                    span { "Projects" }
                                }
                            }
                            li {
                                a href="/team" {
                                    span.icon.is-small {
                                        i.fa.fa-user-friends aria-hidden="true" {}
                                    }
                                    span { "Team" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
