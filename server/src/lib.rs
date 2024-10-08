use dioxus_fullstack::Config;
use web::app;

pub fn server() {
    dioxus_fullstack::launch::launch(app, Vec::new(), Config::new())
}
