use crate::components::toasts::toast::Toast;
use dioxus::prelude::*;

/// 成功トースト通知コンポーネント props
///
/// # Fields
///
/// * `message` - `String` トーストに表示されるメッセージ
/// * `duration` - `usize` トースト表示時間（ミリ秒）
/// * `is_show` - `Signal<bool>` トーストの表示/非表示状態
#[derive(PartialEq, Clone, Props)]
pub struct SuccessToastProps {
    message: String,
    duration: usize,
    is_show: Signal<bool>,
}

/// 成功トースト通知コンポーネント
///
/// # Arguments
///
/// * `props` - `SuccessToastProps` 成功トースト通知コンポーネント props
///
/// # Fields
///
/// * `message` - `String` トーストに表示されるメッセージ
/// * `duration` - `usize` トースト表示時間（ミリ秒）
/// * `is_show` - `Signal<bool>` トーストの表示/非表示状態
pub fn SuccessToast(props: SuccessToastProps) -> Element {
    rsx! {
        Toast {
            message: props.message,
            duration: props.duration,
            is_show: props.is_show,
            bg_color: "bg-green-500",
            svg: rsx! {
                svg {
                    class: "w-5 h-5 mr-2",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    "stroke-width": "1.5",
                    stroke: "currentColor",
                    view_box: "0 0 24 24",

                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                    }
                }
            }
        }
    }
}
