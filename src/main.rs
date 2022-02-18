use dioxus::prelude::*;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let name = "Jon Yardley!";
    let (count, set_count) = use_state(&cx, || 0);
    cx.render(rsx! (
        div {
            h1 { "Hello, {name}" }
            p { "Some body content" }
            h2 { "High-Five counter: {count}" }
            button { onclick: move |_| set_count(count + 1), "Up high!" }
            button { onclick: move |_| set_count(count - 1), "Down low!" }
        }
    ))
}
