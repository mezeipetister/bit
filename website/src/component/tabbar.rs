use crate::component::Component;
use maud::{html, Markup};

pub struct TabBar {}

impl Component for TabBar {
    fn default() -> Markup {
        html! {
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
