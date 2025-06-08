use crate::routes::Route;
use dioxus::prelude::*;

/// ツールカードンポーネント props
///
/// ＃ Fields
///
/// * `to` - `Route` 遷移先ルーティング
/// * `icon` - `Element` ツールのアイコン
/// * `title` - `String` ツールのタイトル
/// * `children` - `Element` ツールの説明など。適切な箇所で改行などを行えるように、子コンポーネントで定義できるよう `Element` 型を受け入れ
#[derive(PartialEq, Clone, Props)]
pub(crate) struct ToolCardProps {
    to: Route,
    icon: Element,
    title: String,
    description: Vec<Vec<String>>,
}

/// ツールコカードンポーネント
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
/// * `children` - `Element` ツールの説明など。適切な箇所で改行などを行えるように、子コンポーネントで定義できるよう `Element` 型を受け入れ
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
