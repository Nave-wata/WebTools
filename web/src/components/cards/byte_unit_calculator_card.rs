use crate::components::cards::tool_card::ToolCard;
use crate::components::images::lazy_image::LazyImage;
use crate::routes::Route;
use dioxus::prelude::*;

/// バイト単位変換ツールカード
pub(crate) fn ByteUnitCalculatorCard() -> Element {
    let icon: Element = rsx! {
        LazyImage {
            src: asset!("/assets/images/icons/byte_unit_calculator.webp"), // アイコン画像
            alt: "バイト単位変換",
            class: "justify-self-center py-3",
            height: 60,
            width: 180,
        }
    };

    rsx! {
        ToolCard {
            to: Route::ByteUnitCalculator {},
            icon: icon,
            title: "バイト単位変換",
            description: vec![
                vec![
                    "バイト単位を相互に".to_string(),
                    "変換することができます。".to_string(),
                ],
                vec![
                    "B | KB | MB | GB | TB".to_string(),
                ]
            ],
        }
    }
}
