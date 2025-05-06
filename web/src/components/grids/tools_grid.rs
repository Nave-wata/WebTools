use dioxus::prelude::*;

/// ツールグリッドコンポーネント props
///
/// # Fields
///
/// * `title` - `String` ツールグリッドのタイトル
/// * `children` - `Element` 関連する各種ツール
#[derive(PartialEq, Clone, Props)]
pub struct ToolsGridProps {
    title: String,
    children: Element,
}

/// ツールグリッドコンポーネント
///
/// # Arguments
///
/// * `props` - `ToolsGridProps` ツールグリッドコンポーネント props
///
/// # Fields
///
/// * `title` - `String` ツールグリッドのタイトル
/// * `children` - `Element` 関連する各種ツール
pub fn ToolsGrid(props: ToolsGridProps) -> Element {
    rsx! {
        div {
            h2 {
                class: "text-xl font-bold",
                {props.title}
            }

            div {
                class: "grid gap-4 mt-3 mb-6",
                {props.children}
            }
        }
    }
}
