use dioxus::prelude::*;

/// 汎用数値入力コンポーネント props
///
/// # Fields
///
/// * `name` - `Option<String>` 入力フィールドの名前
/// * `label` - `Option<String>` 入力フィールドの横に表示されるラベルテキスト
/// * `label_class` - `Option<String>` ラベル要素のCSSクラス
/// * `input_class` - `Option<String>` 入力要素のCSSクラス
/// * `value` - `Option<isize>` 入力フィールドの値
/// * `min` - `Option<isize>` 最小値
/// * `max` - `Option<isize>` 最大値
/// * `is_validate` - `Option<bool>` バリデーション状態
/// * `oninput` - `Option<EventHandler<Event<FormData>>>` 入力時のイベントハンドラー
/// * `onchange` - `Option<EventHandler<Event<FormData>>>` 値変更時のイベントハンドラー
/// * `disabled` - `Option<bool>` 無効状態
#[derive(PartialEq, Clone, Props)]
pub(crate) struct NumberProps {
    name: Option<String>,
    label: Option<String>,
    label_class: Option<String>,
    input_class: Option<String>,
    value: Option<isize>,
    min: Option<isize>,
    max: Option<isize>,
    is_validate: Option<bool>,
    oninput: Option<EventHandler<Event<FormData>>>,
    onchange: Option<EventHandler<Event<FormData>>>,
    disabled: Option<bool>,
}

/// 汎用数値入力コンポーネント
///
/// # Arguments
///
/// * `props` - `NumberProps` 汎用数値入力コンポーネント props
///
/// # Fields
///
/// * `name` - `Option<String>` 入力フィールドの名前
/// * `label` - `Option<String>` 入力フィールドの横に表示されるラベルテキスト
/// * `label_class` - `Option<String>` ラベル要素のCSSクラス
/// * `input_class` - `Option<String>` 入力要素のCSSクラス
/// * `value` - `Option<isize>` 入力フィールドの値
/// * `min` - `Option<isize>` 最小値
/// * `max` - `Option<isize>` 最大値
/// * `is_validate` - `Option<bool>` バリデーション状態
/// * `oninput` - `Option<EventHandler<Event<FormData>>>` 入力時のイベントハンドラー
/// * `onchange` - `Option<EventHandler<Event<FormData>>>` 値変更時のイベントハンドラー
/// * `disabled` - `Option<bool>` 無効状態
pub(crate) fn Number(props: NumberProps) -> Element {
    rsx! {
        label {
            class: props.label_class,

            input {
                r#type: "number",
                name: props.name,
                class: format!("tracking-wider border-1 border-gray-400 rounded-md py-1/2 px-1 {}", props.input_class.unwrap_or_default()),
                value: props.value.unwrap_or_default(),

                min: props.min,
                max: props.max,

                oninput: move |e: Event<FormData>| {
                    if let Some(callback) = props.oninput {
                        callback(e);
                    }
                },
                onchange: move |e: Event<FormData>| {
                    if let Some(callback) = props.onchange {
                        callback(e);
                    }
                },

                disabled: props.disabled,
            }

            {props.label}
        }
    }
}
