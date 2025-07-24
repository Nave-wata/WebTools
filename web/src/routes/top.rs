use crate::components::cards::base_endecode_converter_card::BaseEnDecodeConverterCard;
use crate::components::cards::big_numbers_calculator_card::BigNumbersCalculatorCard;
use crate::components::cards::byte_unit_calculator_card::ByteUnitCalculatorCard;
use crate::components::cards::json_formatter_card::JsonFormatterCard;
use crate::components::cards::number_base_converter_card::NumberBaseConverterCard;
use crate::components::cards::password_generator_card::PasswordGeneratorCard;
use crate::components::cards::simple_calculator_card::SimpleCalculatorCard;
use crate::components::cards::text_length_counter::TextLengthCounterCard;
use crate::components::cards::url_endecode_converter_card::UrlEnDecodeConverterCard;
use crate::components::grids::tools_grid::ToolsGrid;
use crate::components::head::Head;
use crate::constants::app::{APP_TITLE, APP_URL};
use crate::routes::Route;
use dioxus::prelude::*;

/// トップページ
pub(crate) fn TopPage() -> Element {
    let description: &str = "無料でメールアドレスの登録やインストール等一切不要の、幅広い分野で役立つオンラインツールの置き場です。また、個人的な用途のために作成したツールもあるので、マニアックなものまで取り揃えています。";

    rsx! {
        Head {
            title: APP_TITLE,
            description: description,
            og_url: APP_URL,
            og_image: asset!("/assets/images/ogp/top.webp"),
        }

        div {
            class: "px-4",

            ToolsGrid {
                title: "ジェネレータ",
                category_route: Some(Route::GeneratorPage {}),

                PasswordGeneratorCard {},
            }

            ToolsGrid {
                title: "カウンター",
                category_route: Some(Route::CounterPage {}),

                TextLengthCounterCard {},
            }

            ToolsGrid {
                title: "コンバーター",
                category_route: Some(Route::ConverterPage {}),

                UrlEnDecodeConverterCard {},
                BaseEnDecodeConverterCard {},
                NumberBaseConverterCard {},
            }

            ToolsGrid {
                title: "計算機",
                category_route: Some(Route::CalculatorPage {}),

                ByteUnitCalculatorCard {},
                SimpleCalculatorCard {},
                BigNumbersCalculatorCard {},
            }

            ToolsGrid {
                title: "フォーマッター",
                category_route: Some(Route::FormatterPage {}),

                JsonFormatterCard {},
            }
        }
    }
}
