use dioxus::prelude::*;

/// アプリケーションロゴコンポーネントのプロパティ
///
/// # Fields
///
/// * `height` - `i32` ロゴの高さ
/// * `width`  - `i32` ロゴの幅
#[derive(PartialEq, Clone, Props)]
pub(crate) struct AppLogoProps {
    height: i32,
    width: i32,
}

/// アプリケーションロゴコンポーネント
///
/// # Arguments
///
/// * `props` - `AppLogoProps` アプリケーションロゴコンポーネントのプロパティ
pub(crate) fn AppLogo(props: AppLogoProps) -> Element {
    rsx! {
        img {
            src: asset!("/assets/images/logos/app_logo.png"),
            alt: "アプリケーションロゴ",
            height: props.height,
            width: props.width,
        }
    }
}
