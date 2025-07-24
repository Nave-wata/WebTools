pub mod text_length;

use crate::components::breadcrumb::{BreadcrumbItem, BreadcrumbList};
use crate::components::cards::text_length_counter::TextLengthCounterCard;
use crate::components::head::Head;
use crate::routes::Route;
use dioxus::prelude::*;

/// カウンターカテゴリページ
pub(crate) fn CounterPage() -> Element {
    let title = "カウンター";
    let description = "文字数カウントなど、様々なデータを数えるツールです。";

    rsx! {
        Head {
            title: title,
            description: description,
            og_url: Route::CounterPage {}.to_string(),
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
                
                TextLengthCounterCard {},
            }
        }
    }
}
