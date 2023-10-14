use maud::{html, Markup};

use super::root::root;

pub async fn index() -> Markup {
    root(
        "Index",
        html! {
            h1 { "Hello, World!" }
        },
    )
}
