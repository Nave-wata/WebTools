mod top;
mod generator;

use dioxus::prelude::*;
use generator::password::PasswordGenerator;
use top::TopPage;

use crate::components::errors::err_404::Err404;
use crate::components::layouts::default::DefaultLayout;

/// ルーティング定義
#[derive(Clone, Routable, Debug, PartialEq)]
pub(crate) enum Route {
    #[layout(DefaultLayout)]
        #[route("/")]
        TopPage {},

        #[nest("/generator")]
            #[route("/password")]
            PasswordGenerator {},
        #[end_nest]
    #[end_layout]

    #[layout(DefaultLayout)]
    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}
