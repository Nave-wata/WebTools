use dioxus::prelude::*;

/// パスワード生成ツールアイコンポーネントの props
///
/// # Fields
///
/// * `height` - `i32` ロゴの高さ
/// * `width`  - `i32` ロゴの幅
#[derive(PartialEq, Clone, Props)]
pub(crate) struct PasswordGeneratorIconProps {
    height: i32,
    width: i32,
    class: Option<String>,
}

/// パスワード生成ツールアイコンコンポーネント
///
/// # Arguments
///
/// * `props` - `PasswordGeneratorIcon` パスワード生成ツールアイコンコンポーネントの props
///
/// # Fields
///
/// * `height` - `i32` ロゴの高さ
/// * `width`  - `i32` ロゴの幅
pub(crate) fn PasswordGeneratorIcon(props: PasswordGeneratorIconProps) -> Element {
    rsx! {
        img {
            src: asset!("/assets/images/icons/password_generator.png"),
            alt: "パスワード生成",
            class: format!("justify-self-center {}", props.class.unwrap_or_default()),
            height: props.height,
            width: props.width,
        }
    }
}
