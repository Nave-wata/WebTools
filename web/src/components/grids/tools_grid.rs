use dioxus::prelude::*;

/// ツールグリッドコンポーネント props
///
/// # Fields
///
/// * `title` - `String` ツールグリッドのタイトル
/// * `children` - `Element` 関連する各種ツール
#[derive(PartialEq, Clone, Props)]
pub(crate) struct ToolsGridProps {
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
pub(crate) fn ToolsGrid(props: ToolsGridProps) -> Element {
    rsx! {
        div {
            class: "mt-3 mb-15 last:mb-0 max-xl:justify-self-center",

            h2 {
                class: "text-xl font-bold",
                {props.title}
            }

            div {
                class: "grid xl:grid-cols-3 md:grid-cols-2 grid-cols-1 xl:gap-x-4 lg:gap-x-12 gap-x-4 xl:gap-y-8 gap-y-6",
                {props.children}
            }
        }
    }
}
