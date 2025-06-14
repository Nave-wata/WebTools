use dioxus::prelude::*;

/// 汎用ボタンコンポーネント props
///
/// # Fields
///
/// * `class` - `Option<String>` ボタンクラス
/// * `onclick` - `Option<EventHandler<Event<MouseData>>>` クリックイベント
/// * `children` - `Option<Element>` 小要素
#[derive(PartialEq, Clone, Props)]
pub(crate) struct ButtonProps {
    class: Option<String>,
    onclick: Option<EventHandler<Event<MouseData>>>,
    children: Option<Element>,
}

/// 汎用ボタンコンポーネント
///
/// # Arguments
///
/// * `props` - `ButtonProps` 汎用ボタンコンポーネント props
///
/// # Fields
///
/// * `class` - `Option<String>` ボタンクラス
/// * `onclick` - `Option<EventHandler<Event<MouseData>>>` クリックイベント
/// * `children` - `Option<Element>` 小要素
pub(crate) fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: format!("cursor-pointer {}", props.class.unwrap_or("py-1 px-3 border-1 rounded-md".to_string())),
            onclick: move |e| {
                if let Some(onclick) = props.onclick {
                    onclick(e);
                }
            },

            if let Some(children) = props.children {
                {children}
            }
        }
    }
}
