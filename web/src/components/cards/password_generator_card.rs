use crate::components::cards::tool_card::ToolCard;
use crate::components::icons::password_generator_icon::PasswordGeneratorIcon;
use crate::routes::Route;
use dioxus::prelude::*;

/// パスワード生成ツールカード
pub fn PasswordGeneratorCard() -> Element {
    let icon: Element = rsx! {
        PasswordGeneratorIcon {
            height: 60,
            width: 150,
            class: "py-3",
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
