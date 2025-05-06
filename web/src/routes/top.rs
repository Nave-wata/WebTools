use crate::components::cards::password_generator_card::PasswordGeneratorCard;
use crate::components::grids::tools_grid::ToolsGrid;
use crate::components::head::ogp::OGP;
use crate::constants::app::APP_TITLE;
use dioxus::document::{Meta, Title};
use dioxus::prelude::*;

/// トップページ
pub fn TopPage() -> Element {
    let description: &str = "無料でメールアドレスの登録やインストール等一切不要の、幅広い分野で役立つオンラインツールの置き場です。また、個人的な用途のために作成したツールもあるので、マニアックなものまで取り揃えています。";

    rsx! {
        // head
        Title {
            {APP_TITLE}
        }
        Meta {
            name: "description",
            content: description,
        }
        OGP {
            title: APP_TITLE,
            description: description,
            url: APP_TITLE,
            image: format!("{}/favicon.ico", APP_TITLE),
        }

        // body
        ToolsGrid {
            title: "ジェネレータ",

            PasswordGeneratorCard {},
        }
    }
}
