use crate::components::cards::tool_card::ToolCard;
use crate::components::images::lazy_image::LazyImage;
use crate::routes::Route;
use dioxus::prelude::*;

/// 文字数カウントツールカード
pub(crate) fn TextLengthCounterCard() -> Element {
    let icon: Element = rsx! {
        LazyImage {
            src: asset!("/assets/images/icons/text_length_counter.webp"),
            alt: "文字数カウント",
            class: "justify-self-center py-3",
            height: 90,
            width: 70,
        }
    };

    rsx! {
        ToolCard {
            to: Route::TextLengthCounter {},
            icon: icon,
            title: "文字数カウント",
            description: vec![
                vec![
                    "入力された文字列において".to_string(),
                    "次の内容をカウントします。".to_string(),
                ],
                vec![
                    "文字数 | 単語数 | 行数 | バイト数".to_string(),
                ]
            ],
        }
    }
}
