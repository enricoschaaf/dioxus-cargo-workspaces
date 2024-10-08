use dioxus::prelude::*;

pub fn app() -> Element {
    let mut num = use_signal(|| 0);

    rsx! {
        div {
            "hello axum {num}"
            button { onclick: move |_| num += 1, "Increment" }
        }
    }
}

pub fn web() {
    dioxus_web::launch::launch_cfg(app, dioxus_web::Config::new().hydrate(true));
}
