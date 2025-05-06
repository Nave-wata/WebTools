use crate::components::breadcrumb::{BreadcrumbItem, BreadcrumbList};
use crate::components::head::ogp::OGP;
use crate::constants::app::{APP_TITLE, APP_URL};
use crate::routes::Route;
use dioxus::document::{Meta, Title};
use dioxus::prelude::*;

/// 404 エラーページ
///
/// # Arguments
///
/// * `segments` - `Vec<String>` / 区切りの uri
#[component]
pub(crate) fn Err404(segments: Vec<String>) -> Element {
    let title: &str = &format!("404 Not Found - {}", APP_TITLE);
    let description: &str = "お探しのページはすでに削除もしくは移動された可能性があります。URLにミスがないか再度ご確認ください。";

    rsx! {
        // head
        Title {
            {title}
        }
        Meta {
            name: "description",
            content: description,
        }
        Meta {
            name: "robots",
            content: "noindex",
        }
        OGP {
            title: title,
            description: description,
            url: format!("{}/404", APP_URL),
            image: format!("{}/favicon.ico", APP_URL),
        }

        // body
        BreadcrumbList {
            items: vec![
                BreadcrumbItem {
                    name: "トップ".to_string(),
                    to: Some(Route::TopPage {}),
                },
                BreadcrumbItem {
                    name: "404 Not Found".to_string(),
                    to: None,
                }
            ]
        }

        div {
            class: "sm:pt-32 pt-20 flex flex-col items-center justify-center",

            h1 {
                class: "sm:text-5xl text-2xl font-black sm:mb-12 mb-8",
                "404 NOT FOUND"
            }
            p {
                class: "[&_span]:inline-block sm:text-lg text-base text-center",
                span {
                    "お探しのページはすでに削除もしくは"
                }
                span {
                    "移動された可能性があります。"
                }
            }
            p {
                class: "sm:text-lg text-base text-center",
                "URLにミスがないか再度ご確認ください。",
            }

            Link {
                to: Route::TopPage {},
                class: "bg-white mt-12 py-3 px-6 border rounded-3xl",
                "トップページへ戻る"
            }
        }
    }
}
