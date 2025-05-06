use dioxus::prelude::*;

/// ラジオボタンコンポーネント props
///
/// # Fields
///
/// * `name` - `Option<String>` 入力フィールドの名前
/// * `label` - `Option<String>` ラジオボタンの横に表示されるラベルテキスト
/// * `label_class` - `Option<String>` ラベル要素のCSSクラス
/// * `input_class` - `Option<String>` 入力要素のCSSクラス
/// * `value` - `Option<String>` ラジオボタンの値
/// * `checked` - `Option<bool>` チェック状態
/// * `onchange` - `Option<EventHandler<Event<FormData>>>` 値変更時のイベントハンドラー
#[derive(PartialEq, Clone, Props)]
pub(crate) struct RadioProps {
    name: Option<String>,
    label: Option<String>,
    label_class: Option<String>,
    input_class: Option<String>,
    value: Option<String>,
    checked: Option<bool>,
    onchange: Option<EventHandler<Event<FormData>>>,
}

/// ラジオボタンコンポーネント
///
/// # Arguments
///
/// * `props` - `RadioProps` ラジオボタンコンポーネント props
///
/// # Fields
///
/// * `name` - `Option<String>` 入力フィールドの名前
/// * `label` - `Option<String>` ラジオボタンの横に表示されるラベルテキスト
/// * `label_class` - `Option<String>` ラベル要素のCSSクラス
/// * `input_class` - `Option<String>` 入力要素のCSSクラス
/// * `value` - `Option<String>` ラジオボタンの値
/// * `checked` - `Option<bool>` チェック状態
/// * `onchange` - `Option<EventHandler<Event<FormData>>>` 値変更時のイベントハンドラー
pub(crate) fn Radio(props: RadioProps) -> Element {
    rsx! {
        label {
            class: props.label_class,

            input {
                r#type: "radio",
                name: props.name,
                class: props.input_class.unwrap_or("scale-130 m-1 mr-2".to_string()),
                value: props.value,
                checked: props.checked,

                onchange: move |e: Event<FormData>| {
                    if let Some(onchange) = props.onchange {
                        onchange(e);
                    }
                }
            }

            {props.label}
        }
    }
}
