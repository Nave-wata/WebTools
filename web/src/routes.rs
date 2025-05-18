mod top;
mod generator;
mod notfound;

use dioxus::prelude::*;
use generator::password::PasswordGenerator;
use notfound::NotFound;
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
    #[route("/404")]
    NotFound {},

    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}

// The server function at the endpoint "static_routes" will be called by the CLI to generate the list of static
// routes. You must explicitly set the endpoint to `"static_routes"` in the server function attribute instead of
// the default randomly generated endpoint.
#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    // The `Routable` trait has a `static_routes` method that returns all static routes in the enum
    Ok(Route::static_routes().iter().map(ToString::to_string).collect())
}
