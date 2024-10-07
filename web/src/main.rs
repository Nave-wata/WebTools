#![allow(non_snake_case)]

mod routes;
mod components;

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

use routes::Route;

/// エントリーポイント
fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    launch(App);
}

/// アプリケーションコンポーネント
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
