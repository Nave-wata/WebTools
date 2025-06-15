use dioxus::prelude::*;

/// 使い方セクションのプロパティを表す構造体
///
/// # Fields
///
/// * `title` - セクションのタイトル
/// * `items` - セクション内の項目リスト
#[derive(PartialEq, Clone, Props)]
pub struct UsageSectionProps {
    pub title: String,
    pub items: Vec<String>,
}

/// 使い方セクションコンポーネント
///
/// # Arguments
///
/// * `props` - 使い方セクションコンポーネントのプロパティ
///
/// # Fields
///
/// * `title` - セクションのタイトル
/// * `items` - セクション内の項目リスト
pub fn UsageSection(props: UsageSectionProps) -> Element {
    rsx! {
        h3 {
            class: "font-bold mb-2",
            "{props.title}"
        }
        ul {
            class: "list-disc pl-5 space-y-2 mb-4",
            for item in &props.items {
                li { "{item}" }
            }
        }
    }
}

/// 使い方コンポーネントのプロパティを表す構造体
///
/// # Fields
///
/// * `title` - 使い方のタイトル（デフォルトは "使い方"）
/// * `sections` - 使い方の各セクション
/// * `class` - 追加のCSSクラス（オプション）
#[derive(PartialEq, Clone, Props)]
pub struct UsageProps {
    #[props(default = "使い方".to_string())]
    pub title: String,
    pub sections: Vec<UsageSectionProps>,
    #[props(default)]
    pub class: Option<String>,
}

/// 使い方コンポーネント
///
/// このコンポーネントは、ツールの使い方を表示するための再利用可能なコンポーネントです。
/// 各ツールページで共通のスタイルと構造を持つ使い方セクションを簡単に追加できます。
///
/// # Arguments
///
/// * `props` - 使い方コンポーネントのプロパティ
///
/// # Fields
///
/// * `title` - 使い方のタイトル（デフォルトは "使い方"）
/// * `sections` - 使い方の各セクション
/// * `class` - 追加のCSSクラス（オプション）
pub fn Usage(props: UsageProps) -> Element {
    let container_class = format!("mt-8 mb-6 {}", props.class.clone().unwrap_or_default());

    rsx! {
        div {
            class: container_class,
            h2 {
                class: "text-xl font-bold mb-3 text-center",
                "{props.title}"
            }

            div {
                class: "bg-gray-50 p-4 rounded-md text-left mx-auto max-w-3xl",
                for section in &props.sections {
                    UsageSection {
                        title: section.title.clone(),
                        items: section.items.clone(),
                    }
                }
            }
        }
    }
}
