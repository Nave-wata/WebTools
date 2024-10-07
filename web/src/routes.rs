mod top;

use dioxus::prelude::*;
use top::TopPage;

use crate::components::layouts::default::DefaultLayout;

/// ルーティング定義
#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[layout(DefaultLayout)]
        #[route("/")]
        TopPage {},
}
