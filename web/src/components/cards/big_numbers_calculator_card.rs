use crate::components::cards::tool_card::ToolCard;
use crate::components::images::lazy_image::LazyImage;
use crate::routes::Route;
use dioxus::prelude::*;

/// 大きな数値の電卓ツールカード
pub(crate) fn BigNumbersCalculatorCard() -> Element {
    let icon: Element = rsx! {
        LazyImage {
            src: asset!("/assets/images/icons/big_numbers_calculator.webp"),
            alt: "大きな数値の電卓",
            class: "justify-self-center py-3",
            height: 110,
            width: 90,
        }
    };

    rsx! {
        ToolCard {
            to: Route::BigNumbersCalculator {},
            icon: icon,
            title: "大きな数値の計算",
            description: vec![
                vec![
                    "非常に大きな数値での".to_string(),
                    "計算に対応した計算機です。".to_string(),
                ],
                vec![
                    "通常の電卓では扱えない".to_string(),
                    "大きな数値でも計算できます。".to_string(),
                ]
            ],
        }
    }
}
