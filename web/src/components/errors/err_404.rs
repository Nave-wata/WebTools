use dioxus::document::{Meta, Title};
use dioxus::prelude::*;
use crate::components::breadcrumb::{BreadcrumbList, BreadcrumbItem};
use crate::components::head::ogp::OGP;
use crate::constants::app::APP_TITLE;
use crate::routes::Route;

/// 404 エラーページ
///
/// # Arguments
///
/// * `segments` - `Vec<String>` / 区切りの uri
#[component]
pub fn Err404(segments: Vec<String>) -> Element {
    let title: &str = &format!("404 Not Found - {}", APP_TITLE);

    rsx! {
        Title {
            {title}
        }
        OGP {
            title: {title},
        }

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
