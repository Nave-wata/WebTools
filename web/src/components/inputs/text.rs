use dioxus::prelude::*;

/// 汎用テキスト入力コンポーネント props
///
/// # Fields
///
/// * `name` - `Option<String>` 入力フィールドの名前
/// * `label` - `Option<String>` 入力フィールドの横に表示されるラベルテキスト
/// * `label_class` - `Option<String>` ラベル要素のCSSクラス
/// * `input_class` - `Option<String>` 入力要素のCSSクラス
/// * `value` - `Option<String>` 入力フィールドの値
/// * `oninput` - `Option<EventHandler<Event<FormData>>>` 入力時のイベントハンドラー
/// * `onchange` - `Option<EventHandler<Event<FormData>>>` 値変更時のイベントハンドラー
#[derive(PartialEq, Clone, Props)]
pub(crate) struct TextProps {
    name: Option<String>,
    label: Option<String>,
    label_class: Option<String>,
    input_class: Option<String>,
    value: Option<String>,
    oninput: Option<EventHandler<Event<FormData>>>,
    onchange: Option<EventHandler<Event<FormData>>>,
}

/// 汎用テキスト入力コンポーネント
///
/// # Arguments
///
/// * `props` - `TextProps` 汎用テキスト入力コンポーネント props
///
/// # Fields
///
/// * `name` - `Option<String>` 入力フィールドの名前
/// * `label` - `Option<String>` 入力フィールドの横に表示されるラベルテキスト
/// * `label_class` - `Option<String>` ラベル要素のCSSクラス
/// * `input_class` - `Option<String>` 入力要素のCSSクラス
/// * `value` - `Option<String>` 入力フィールドの値
/// * `oninput` - `Option<EventHandler<Event<FormData>>>` 入力時のイベントハンドラー
/// * `onchange` - `Option<EventHandler<Event<FormData>>>` 値変更時のイベントハンドラー
pub(crate) fn Text(props: TextProps) -> Element {
    rsx! {
        label {
            class: props.label_class,

            input {
                r#type: "text",
                name: props.name,
                class: format!("tracking-wider border-1 border-gray-400 rounded-md p-1/2 {}", props.input_class.unwrap_or_default()),
                value: props.value,

                oninput: move |e: Event<FormData>| {
                    if let Some(oninput) = props.oninput {
                        oninput(e);
                    }
                },
                onchange: move |e: Event<FormData>| {
                    if let Some(onchange) = props.onchange {
                        onchange(e);
                    }
                },
            }

            {props.label.unwrap_or_default()}
        }
    }
}
