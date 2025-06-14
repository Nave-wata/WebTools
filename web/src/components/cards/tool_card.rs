use crate::routes::Route;
use dioxus::prelude::*;

/// ツールカードコンポーネント props
///
/// ＃ Fields
///
/// * `to` - `Route` 遷移先ルーティング
/// * `icon` - `Element` ツールのアイコン
/// * `title` - `String` ツールのタイトル
/// * `description` - `Vec<Vec<String>>` ツールの説明。行と単語のネストされた配列
#[derive(PartialEq, Clone, Props)]
pub(crate) struct ToolCardProps {
    to: Route,
    icon: Element,
    title: String,
    description: Vec<Vec<String>>,
}

/// ツールカードコンポーネント
///
/// # Arguments
///
/// - `props` - `ToolCardProps` ツールカードコンポーネント props
///
/// # Fields
///
/// * `to` - `Route` 遷移先ルーティング
/// * `icon` - `Element` ツールのアイコン
/// * `title` - `String` ツールのタイトル
/// * `description` - `Vec<Vec<String>>` ツールの説明。行と単語のネストされた配列
pub(crate) fn ToolCard(props: ToolCardProps) -> Element {
    let description = props.description.iter().map(|line| {
        rsx! {
            span {
                {line.iter().map(|word| {
                    rsx! {
                        span {
                            {word.clone()}
                        }
                    }
                })}
            }
        }
    });

    rsx! {
        div {
            class: "bg-white text-center min-h-[185px] max-w-[400px] rounded-lg",

            Link {
                to: props.to.clone(),

                {props.icon}

                h3 {
                    class: "my-3 text-xl font-bold",
                    {props.title}
                }

                p {
                    class: "pb-3 px-2 [&_span]:inline-block [&_span]:text-sm [&_span]:font-light",
                    {description}
                }
            }
        }
    }
}
