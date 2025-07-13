use crate::components::cards::tool_card::ToolCard;
use crate::components::images::lazy_image::LazyImage;
use crate::routes::Route;
use dioxus::prelude::*;

/// 進数変換ツールカード
pub(crate) fn NumberBaseConverterCard() -> Element {
    let icon: Element = rsx! {
        LazyImage {
            src: asset!("/assets/images/icons/number_base_converter.webp"), // 仮のアイコン画像
            alt: "進数変換",
            class: "justify-self-center py-3",
            height: 100,
            width: 100,
        }
    };

    rsx! {
        ToolCard {
            to: Route::NumberBaseConverter {},
            icon: icon,
            title: "進数変換",
            description: vec![
                vec![
                    "数値の進数を相互に".to_string(),
                    "変換することができます。".to_string(),
                ],
                vec![
                    "2進数 | 10進数 | 16進数".to_string(),
                ]
            ],
        }
    }
}
