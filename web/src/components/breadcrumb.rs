use dioxus::prelude::*;
use crate::routes::Route;

/// パンくずリスト props
///
/// # Fields
///
/// * `items` - `Vec<BreadcrumbItem>` パンくずリストのアイテムベクタ
#[derive(PartialEq, Clone, Props)]
pub struct BreadcrumbListProps {
    /// パンくずリストのアイテムベクタ
    items: Vec<BreadcrumbItem>,
}

/// パンくずリスト item
///
/// # Fields
///
/// * `name` - `String` 表示する要素名
/// * `to` - `Option<Route>` たどってきたページのリンク
#[derive(PartialEq, Clone, Props)]
pub struct BreadcrumbItem {
    /// 表示する要素名
    pub name: String,
    /// 遷移するページルート
    pub to: Option<Route>,
}

/// パンくずリストコンポーネント
///
/// # Arguments
///
/// * `props` - `BreadcrumbListProps` パンくずリストのアイテムベクタ
///
/// # Fields
///
/// * `items` - `Vec<BreadcrumbItem>` パンくずリストのアイテムベクタ
pub fn BreadcrumbList(props: BreadcrumbListProps) -> Element {
    rsx! {
        ol {
            itemtype: "https://schema.org/BreadcrumbList",
            itemscope: true,
            class: "flex pt-[-2rem]",

            {props.items
                .iter()
                .enumerate()
                .map(|(index, item)| rsx! {
                    BreadcrumbItemElement {
                        name: item.name.clone(),
                        to: item.to.clone(),
                    }

                    if index != props.items.len() - 1 {
                        span {
                            class: "relative self-center top-[1px] mx-2 text-xs",
                            ">"
                        }
                    }
                })
            }
        }
    }
}

/// パンくずリストのアイテムコンポーネント
///
/// # Arguments
///
/// * `props` - `BreadcrumbItem` パンくずリストのアイテム構造体
///
/// # Fields
///
/// * `name` - `String` 表示する要素名
/// * `to` - `Option<Route>` たどってきたページのリンク
fn BreadcrumbItemElement(props: BreadcrumbItem) -> Element {
    rsx! {
        li {
            itemprop: "itemListElement",
            itemscope: true,
            itemtype: "https://schema.org/ListItem",

            if let Some(to) = props.to {
                Link {
                    to: to,
                    class: "self-center text-blue-400 text-xs",

                    {props.name.clone()}
                }
            } else {
                span {
                    class: "self-center text-xs",
                    {props.name.clone()}
                }
            }

            // meta タグは現状 head タグに行ってしまうため配置しない
        }
    }
}
