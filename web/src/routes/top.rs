use dioxus::document::{Meta, Title};
use dioxus::prelude::*;
use crate::components::head::ogp::OGP;
use crate::constants::app::APP_TITLE;

/// トップページ
pub fn TopPage() -> Element {
    let mut count = use_signal(|| 0);
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
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
