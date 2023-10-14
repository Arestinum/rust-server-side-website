use maud::{html, Markup};

pub fn root(title: &'static str, page: Markup) -> Markup {
    html! {
        meta charset="utf-8";
        title { (title) }
        h2 { "Test" };
        (page)
    }
}
