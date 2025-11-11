use crate::components::cards::tool_card::ToolCard;
use crate::components::images::lazy_image::LazyImage;
use crate::routes::Route;
use dioxus::prelude::*;

pub(crate) fn Uuid4GeneratorCard() -> Element {
    let icon: Element = rsx! {
        LazyImage {
            src: "https://api.nave-wata.net/tools/generator/dummy-image",
            alt: "UUID Version 4 生成",
            class: "justify-self-center py-3",
            height: 60,
            width: 150,
        }
    };

    rsx! {
        ToolCard {
            to: Route::Uuid4Generator {},
            icon: icon,
            title: "UUID Version 4 生成",
            description: vec![
                vec![
                    "UUID Version 4をランダムに".to_string(),
                    "生成することができます。".to_string(),
                ],
                vec![
                    "1個から100個まで、必要な数の".to_string(),
                    "UUIDを一度に生成できます。".to_string(),
                ],
            ],
        }
    }
}
