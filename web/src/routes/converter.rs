pub mod base_endecode;
pub mod number_base;
pub mod url_endecode;

use crate::components::breadcrumb::{BreadcrumbItem, BreadcrumbList};
use crate::components::cards::base_endecode_converter_card::BaseEnDecodeConverterCard;
use crate::components::cards::number_base_converter_card::NumberBaseConverterCard;
use crate::components::cards::url_endecode_converter_card::UrlEnDecodeConverterCard;
use crate::components::head::Head;
use crate::routes::Route;
use dioxus::prelude::*;

/// コンバーターカテゴリページ
pub(crate) fn ConverterPage() -> Element {
    let title = "コンバーター";
    let description = "URLエンコード・デコード、Base64エンコード・デコード、進数変換など、データの変換を行うツールです。";

    rsx! {
        Head {
            title: title,
            description: description,
            og_url: Route::ConverterPage {}.to_string(),
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
                
                UrlEnDecodeConverterCard {},
                BaseEnDecodeConverterCard {},
                NumberBaseConverterCard {},
            }
        }
    }
}
