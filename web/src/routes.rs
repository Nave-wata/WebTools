mod top;

use dioxus::prelude::*;
use top::TopPage;

use crate::components::layouts::default::DefaultLayout;
use crate::components::errors::err_404::Err404;

/// ルーティング定義
#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[layout(DefaultLayout)]
        #[route("/")]
        TopPage {},

        #[route("/:..segments")]
        Err404 { segments: Vec<String> },
}
