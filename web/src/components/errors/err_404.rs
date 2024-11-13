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
            class: "pt-32 flex flex-col items-center justify-center",

            h1 {
                class: "text-5xl font-black mb-12",
                "404 NOT FOUND"
            }
            p {
                class: "text-lg",
                "お探しのページはすでに削除もしくは移動された可能性があります。"
            }
            p {
                class: "text-lg",
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
