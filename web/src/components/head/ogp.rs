use dioxus::document::Meta;
use dioxus::prelude::*;

/// 変更可能な OGP タグのリスト
#[derive(PartialEq, Clone, Props)]
pub struct OgpProps {
    /// `og:title` を指定された値に変更する
    pub title: Option<String>,
    /// `og:description` を指定された値に変更する
    pub description: Option<String>,
    /// `og:url` を指定された値に変更する
    pub url: Option<String>,
    /// `og:image` を指定された値に変更する
    pub image: Option<String>,
}

/// OGP タグをまとめて定義するコンポーネント
/// 指定された値のみ初期値から更新する
///
/// # Attributes
///
/// * `props` - `OgpProps` 変更したい OGP の値
///
/// # Fields
///
/// * `title` - `Option<String>` og:title を指定された値に変更する
/// * `description` - `Option<String>` og:description を指定された値に変更する
/// * `url` - `Option<String>` og:url を指定された値に変更する
/// * `image` - `Option<String>` og:image を指定された値に変更する
pub fn OGP(props: OgpProps) -> Element {
    rsx! {
        if let Some(title) = props.title {
            Meta {
                name: "og:title",
                content: title
            }
        }

        if let Some(description) = props.description {
            Meta {
                name: "og:description",
                content: description
            }
        }

        if let Some(url) = props.url {
            Meta {
                name: "og:url",
                content: url
            }
        }

        if let Some(image) = props.image {
            Meta {
                name: "og:image",
                content: image
            }
        }
    }
}
