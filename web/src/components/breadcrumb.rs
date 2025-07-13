use crate::routes::Route;
use dioxus::prelude::*;

/// パンくずリスト props
///
/// # Fields
///
/// * `items` - `Vec<BreadcrumbItem>` パンくずリストのアイテムベクタ
#[derive(PartialEq, Clone, Props)]
pub(crate) struct BreadcrumbListProps {
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
pub(crate) struct BreadcrumbItem {
    /// 表示する要素名
    pub(crate) name: String,
    /// 遷移するページルート
    pub(crate) to: Option<Route>,
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
pub(crate) fn BreadcrumbList(props: BreadcrumbListProps) -> Element {
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
                        position: index + 1,
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
/// * `position` - `usize` パンくずリスト内での位置（1から開始）
///
/// # Fields
///
/// * `name` - `String` 表示する要素名
/// * `to` - `Option<Route>` たどってきたページのリンク
/// * `position` - `usize` パンくずリスト内での位置
#[derive(PartialEq, Clone, Props)]
struct BreadcrumbItemElementProps {
    name: String,
    to: Option<Route>,
    position: usize,
}

fn BreadcrumbItemElement(props: BreadcrumbItemElementProps) -> Element {
    rsx! {
        li {
            itemprop: "itemListElement",
            itemscope: true,
            itemtype: "https://schema.org/ListItem",

            // Use meta tag for position, which is cleaner for non-visible data
            meta {
                itemprop: "position",
                content: "{props.position}"
            }

            if let Some(to) = props.to {
                // For links, the <a> tag is the "item" (the URL) and the text inside is the "name"
                Link {
                    to: to,
                    class: "self-center text-blue-400 text-xs",
                    itemprop: "item",
                    span {
                        itemprop: "name",
                        {props.name.clone()}
                    }
                }
            } else {
                // For the current page (not a link), we only provide the "name"
                span {
                    class: "self-center text-xs",
                    itemprop: "name",
                    {props.name.clone()}
                }
            }
        }
    }
}
