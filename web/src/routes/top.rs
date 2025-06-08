use crate::components::cards::password_generator_card::PasswordGeneratorCard;
use crate::components::cards::text_length_counter::TextLengthCounterCard;
use crate::components::grids::tools_grid::ToolsGrid;
use crate::components::head::Head;
use crate::constants::app::{APP_TITLE, APP_URL};
use dioxus::prelude::*;

/// トップページ
pub(crate) fn TopPage() -> Element {
    let description: &str = "無料でメールアドレスの登録やインストール等一切不要の、幅広い分野で役立つオンラインツールの置き場です。また、個人的な用途のために作成したツールもあるので、マニアックなものまで取り揃えています。";

    rsx! {
        Head {
            title: APP_TITLE,
            description: description,
            og_url: APP_URL,
        }

        ToolsGrid {
            title: "ジェネレータ",

            PasswordGeneratorCard {},
        }

        ToolsGrid {
            title: "カウンター",

            TextLengthCounterCard {},
        }
    }
}
