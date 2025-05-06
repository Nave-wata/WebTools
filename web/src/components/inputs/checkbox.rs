use dioxus::prelude::*;

/// 汎用チェックボックスコンポーネント props
///
/// # Fields
///
/// * `name` - `Option<String>` 入力フィールドの名前
/// * `label` - `Option<String>` チェックボックスの横に表示されるラベルテキスト
/// * `label_class` - `Option<String>` ラベル要素のCSSクラス
/// * `input_class` - `Option<String>` 入力要素のCSSクラス
/// * `value` - `Option<String>` 入力フィールドの値
/// * `checked` - `Option<bool>` チェックボックスのチェック状態
/// * `onchange` - `Option<EventHandler<Event<FormData>>>` 値変更時のイベントハンドラー
#[derive(PartialEq, Clone, Props)]
pub struct CheckboxProps {
    name: Option<String>,
    label: Option<String>,
    label_class: Option<String>,
    input_class: Option<String>,
    value: Option<String>,
    checked: Option<bool>,
    onchange: Option<EventHandler<Event<FormData>>>,
}


/// 汎用チェックボックスコンポーネント
///
/// # Arguments
///
/// * `props` - `CheckboxProps` 汎用チェックボックスコンポーネント props
///
/// # Fields
///
/// * `name` - `Option<String>` 入力フィールドの名前
/// * `label` - `Option<String>` チェックボックスの横に表示されるラベルテキスト
/// * `label_class` - `Option<String>` ラベル要素のCSSクラス
/// * `input_class` - `Option<String>` 入力要素のCSSクラス
/// * `value` - `Option<String>` 入力フィールドの値
/// * `checked` - `Option<bool>` チェックボックスのチェック状態
/// * `onchange` - `Option<EventHandler<Event<FormData>>>` 値変更時のイベントハンドラー
pub fn Checkbox(props: CheckboxProps) -> Element {
    rsx! {
        label {
            class: props.label_class.unwrap_or("mr-3".to_string()),

            input {
                r#type: "checkbox",
                name: props.name,
                class: props.input_class.unwrap_or("scale-130 m-1 mr-2".to_string()),
                value: props.value,
                checked: props.checked,

                onchange: move |e: Event<FormData>| {
                    if let Some(onchange) = props.onchange {
                        onchange(e);
                    }
                },
            }

            {props.label}
        }
    }
}
