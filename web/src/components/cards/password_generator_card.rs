use crate::components::cards::tool_card::ToolCard;
use crate::components::images::lazy_image::LazyImage;
use crate::routes::Route;
use dioxus::prelude::*;

/// パスワード生成ツールカード
pub(crate) fn PasswordGeneratorCard() -> Element {
    let icon: Element = rsx! {
        LazyImage {
            src: asset!("/assets/images/icons/password_generator.webp"),
            alt: "パスワード生成",
            class: "justify-self-center py-3",
            height: 75,
            width: 150,
        }
    };

    rsx! {
        ToolCard {
            to: Route::PasswordGenerator {},
            icon: icon,
            title: "パスワード生成",
            description: vec![
                vec![
                    "任意の条件でパスワードを".to_string(),
                    "生成することができます。".to_string(),
                ],
                vec![
                    "パスワードに使用する英数字や".to_string(),
                    "記号などを選択できます。".to_string(),
                ],
            ],
        }
    }
}
