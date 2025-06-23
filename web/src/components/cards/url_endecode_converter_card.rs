use crate::components::cards::tool_card::ToolCard;
use crate::routes::Route;
use dioxus::prelude::*;

/// URL エンコード/デコードツールカード
pub(crate) fn UrlEnDecodeConverterCard() -> Element {
    let icon: Element = rsx! {
        img {
            src: asset!("/assets/images/icons/url_endecode_converter.webp"), // 仮のアイコン画像
            alt: "URL エンコード/デコード",
            class: "justify-self-center pt-5 pb-6",
            height: 50,
            width: 180,
        }
    };

    rsx! {
        ToolCard {
            to: Route::UrlEndecoder {},
            icon: icon,
            title: "URL エンコード/デコード",
            description: vec![
                vec![
                    "URL のエンコードと".to_string(),
                    "デコードができます。".to_string(),
                ],
                vec![
                    "日本語 ⇔ %E6%97%A5%E6%9C%AC%E8%AA%9E".to_string(),
                ]
            ],
        }
    }
}
