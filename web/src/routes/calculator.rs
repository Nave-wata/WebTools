pub mod big_numbers;
pub mod byte_unit;
pub mod simple;

use crate::components::breadcrumb::{BreadcrumbItem, BreadcrumbList};
use crate::components::cards::big_numbers_calculator_card::BigNumbersCalculatorCard;
use crate::components::cards::byte_unit_calculator_card::ByteUnitCalculatorCard;
use crate::components::cards::simple_calculator_card::SimpleCalculatorCard;
use crate::components::head::Head;
use crate::routes::Route;
use dioxus::prelude::*;

/// 計算機カテゴリページ
pub(crate) fn CalculatorPage() -> Element {
    let title = "計算機";
    let description = "バイト単位計算機、シンプル計算機、巨大数値計算機など、様々な計算を行うツールです。";

    rsx! {
        Head {
            title: title,
            description: description,
            og_url: Route::CalculatorPage {}.to_string(),
            og_image: asset!("/assets/images/ogp/top.webp"),
        }

        BreadcrumbList {
            items: vec![
                BreadcrumbItem {
                    name: "トップ".to_string(),
                    to: Some(Route::TopPage {})
                },
                BreadcrumbItem {
                    name: title.to_string(),
                    to: None,
                }
            ]
        }

        div {
            class: "px-4",

            h1 {
                class: "my-5 text-3xl max-sm:text-2xl font-bold",
                {title}
            }

            p {
                class: "mb-8 text-gray-600",
                {description}
            }

            div {
                class: "grid xl:grid-cols-3 md:grid-cols-2 grid-cols-1 xl:gap-x-4 lg:gap-x-12 gap-x-4 xl:gap-y-8 gap-y-6",
                
                ByteUnitCalculatorCard {},
                SimpleCalculatorCard {},
                BigNumbersCalculatorCard {},
            }
        }
    }
}
