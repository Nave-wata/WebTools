use crate::components::images::lazy_image::LazyImage;
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
        LazyImage {
            src: asset!("/assets/images/logos/app_logo.webp"),
            alt: "アプリケーションロゴ",
            height: props.height,
            width: props.width,
            show_placeholder: false, // Logo should load immediately for branding
        }
    }
}
