use crate::components::cards::tool_card::ToolCard;
use crate::routes::Route;
use dioxus::prelude::*;

/// JSON フォーマッターツールカード
pub(crate) fn JsonFormatterCard() -> Element {
    let icon: Element = rsx! {
        img {
            src: asset!("/assets/images/icons/json_formatter.webp"),
            alt: "JSON フォーマッター",
            class: "justify-self-center py-3",
            height: 150,
            width: 150,
        }
    };

    rsx! {
        ToolCard {
            to: Route::JsonFormatter {},
            icon: icon,
            title: "JSON フォーマッター",
            description: vec![
                vec![
                    "JSONデータの整形・圧縮が".to_string(),
                    "できます。".to_string(),
                ],
                vec![
                    "構文チェック機能付きで".to_string(),
                    "安全にフォーマットできます。".to_string(),
                ]
            ],
        }
    }
}
