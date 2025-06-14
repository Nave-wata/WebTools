use crate::constants::app::{APP_TITLE, APP_URL};
use dioxus::document::{Meta, Title};
use dioxus::prelude::*;

/// ヘッドコンポーネントのプロパティ
///
/// # Fields
///
/// * `title` - ページのタイトル
/// * `description` - ページの説明
/// * `robots` - robots metaタグの内容（オプション）
/// * `og_url` - OGPのURL（オプション）
/// * `og_image` - OGPの画像パス（オプション）
/// * `og_description` - OGPの説明（オプション）
#[derive(PartialEq, Clone, Props)]
pub(crate) struct HeadProps {
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) robots: Option<String>,
    pub(crate) og_url: Option<String>,
    pub(crate) og_image: Option<String>,
    pub(crate) og_description: Option<String>,
}

/// ヘッドコンポーネント
///
/// # Arguments
///
/// * `props` - ヘッドコンポーネントのプロパティ
///
/// # Fields
///
/// * `title` - ページのタイトル
/// * `description` - ページの説明
/// * `robots` - robots metaタグの内容（オプション）
/// * `og_url` - OGPのURL（オプション）
/// * `og_image` - OGPの画像パス（オプション）
/// * `og_description` - OGPの説明（オプション）
///
/// # Returns
///
/// * `Element` HTMLのheadタグ内に配置するメタ情報を含むElement
pub(crate) fn Head(props: HeadProps) -> Element {
    let page_title = if props.title.contains(APP_TITLE) {
        props.title.clone()
    } else {
        format!("{} - {}", props.title, APP_TITLE)
    };
    let og_description = match props.og_description {
        Some(og_description) => og_description,
        None => props.description.clone(),
    };

    // 既存の meta タグなどを削除
    document::eval(
        r#"
        const originalPushState = history.pushState;
        history.pushState = function (...args) {
            for (const selector of [
                "meta[name='description']",
                "meta[name='robots']",
                "meta[name='og:title']",
                "meta[name='og:description']",
                "meta[name='og:url']",
                "meta[name='og:image']"
            ]) {
                document.querySelectorAll(selector).forEach(el => el.remove());
            }
            
            originalPushState.apply(this, args);
        };
    "#,
    );

    // 各ページ専用の meta 情報などを配置
    rsx! {
        Title {
            {page_title.clone()}
        }

        Meta {
            name: "description",
            content: props.description.clone(),
        }

        if let Some(robots) = props.robots {
            Meta {
                name: "robots",
                content: robots,
            }
        }

        Meta {
            name: "og:title",
            content: page_title
        }
        Meta {
            name: "og:description",
            content: og_description,
        }

        if let Some(url) = props.og_url {
            Meta {
                name: "og:url",
                content: format!("{}{}", APP_URL, url),
            }
        }

        if let Some(image) = props.og_image {
            Meta {
                name: "og:image",
                content: format!("{}{}", APP_URL, image)
            }
        }
    }
}
