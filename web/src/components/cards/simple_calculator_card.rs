use crate::components::cards::tool_card::ToolCard;
use crate::routes::Route;
use dioxus::prelude::*;

/// シンプル電卓ツールカード
pub(crate) fn SimpleCalculatorCard() -> Element {
    let icon: Element = rsx! {
        img {
            src: asset!("/assets/images/icons/simple_calculator.webp"),
            alt: "シンプル電卓",
            class: "justify-self-center py-3",
            height: 60,
            width: 50,
        }
    };

    rsx! {
        ToolCard {
            to: Route::SimpleCalculator {},
            icon: icon,
            title: "シンプル電卓",
            description: vec![
                vec![
                    "基本的な四則演算を".to_string(),
                    "行うことができます。".to_string(),
                ],
                vec![
                    "加算 | 減算 | 乗算 | 除算".to_string(),
                ]
            ],
        }
    }
}
