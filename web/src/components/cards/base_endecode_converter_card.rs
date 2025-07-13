use crate::components::cards::tool_card::ToolCard;
use crate::components::images::lazy_image::LazyImage;
use crate::routes::Route;
use dioxus::prelude::*;

/// Base エンコード/デコードツールカード
pub(crate) fn BaseEnDecodeConverterCard() -> Element {
    let icon: Element = rsx! {
        LazyImage {
            src: asset!("/assets/images/icons/base_endecode_converter.webp"),
            alt: "Base エンコード/デコード",
            class: "justify-self-center py-3",
            height: 150,
            width: 150,
        }
    };

    rsx! {
        ToolCard {
            to: Route::BaseEnDecoder {},
            icon: icon,
            title: "Base エンコード/デコード",
            description: vec![
                vec![
                    "Base16, Base32, Base64の".to_string(),
                    "エンコード/デコードができます。".to_string(),
                ],
                vec![
                    "Hello ⇔ SGVsbG8= (Base64)".to_string(),
                ]
            ],
        }
    }
}
