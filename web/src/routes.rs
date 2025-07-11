mod calculator;
mod converter;
mod counter;
mod formatter;
mod generator;
mod notfound;
mod top;

use calculator::big_numbers::BigNumbersCalculator;
use calculator::byte_unit::ByteUnitCalculator;
use calculator::simple::SimpleCalculator;
use converter::base_endecode::BaseEnDecoder;
use converter::number_base::NumberBaseConverter;
use converter::url_endecode::UrlEnDecoder;
use counter::text_length::TextLengthCounter;
use dioxus::prelude::*;
use formatter::json::JsonFormatter;
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
    #[nest("/counter")]
    #[route("/text-length")]
    TextLengthCounter {},
    #[end_nest]
    #[nest("/converter")]
    #[route("/base-endecode")]
    BaseEnDecoder {},

    #[route("/number-base")]
    NumberBaseConverter {},

    #[route("/url-endecode")]
    UrlEnDecoder {},
    #[end_nest]
    #[nest("/calculator")]
    #[route("/byte-unit")]
    ByteUnitCalculator {},

    #[route("/simple")]
    SimpleCalculator {},

    #[route("/big-numbers")]
    BigNumbersCalculator {},
    #[end_nest]
    #[nest("/formatter")]
    #[route("/json")]
    JsonFormatter {},
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
#[cfg(feature = "production")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    // The `Routable` trait has a `static_routes` method that returns all static routes in the enum
    Ok(Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect())
}
