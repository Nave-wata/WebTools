use crate::components::breadcrumb::{BreadcrumbItem, BreadcrumbList};
use crate::components::head::Head;
use crate::components::instructions::usage::{Usage, UsageSectionProps};
use crate::libs::url::{url_decode, url_encode};
use crate::routes::Route;
use dioxus::prelude::*;

/// URL エンコード/デコードツールページコンポーネント
///
/// このコンポーネントは以下の機能を提供します：
/// * URL のエンコードとデコード
pub(crate) fn UrlEndecoder() -> Element {
    let title: &str = "URL エンコード/デコード";
    let description: &str = "URL のエンコードとデコードを行うツールです。URL で使用できない文字をエンコードしたり、エンコードされた URL をデコードしたりすることができます。";

    // 通常テキストとエンコード済みテキスト
    let mut normal_text = use_signal(String::new);
    let mut encoded_text = use_signal(String::new);

    // 最後に編集されたフィールド（normal または encoded）
    let mut last_edited = use_signal(|| "");

    // エラーメッセージ
    let mut error_message = use_signal(String::new);

    // 通常テキスト入力処理
    let on_normal_input = move |e: Event<FormData>| {
        let value = e.value().to_string();
        normal_text.set(value.clone());
        last_edited.set("normal");
        error_message.set(String::new());

        // 入力が空の場合はエンコードテキストもクリア
        if value.is_empty() {
            encoded_text.set(String::new());
            return;
        }

        // リアルタイムでエンコード
        encoded_text.set(url_encode(&value));
    };

    // エンコード済みテキスト入力処理
    let on_encoded_input = move |e: Event<FormData>| {
        let value = e.value().to_string();
        encoded_text.set(value.clone());
        last_edited.set("encoded");
        error_message.set(String::new());

        // 入力が空の場合は通常テキストもクリア
        if value.is_empty() {
            normal_text.set(String::new());
            return;
        }

        // リアルタイムでデコード
        match url_decode(&value) {
            Ok(result) => normal_text.set(result),
            Err(err) => error_message.set(format!("デコードエラー: {}", err)),
        }
    };

    rsx! {
        Head {
            title: title,
            description: description,
            og_url: Route::UrlEndecoder {}.to_string(),
            og_image: asset!("/assets/images/ogp/url_endecode_converter.webp"),
        }

        BreadcrumbList {
            items: vec! [
                BreadcrumbItem {
                    name: "トップ".to_string(),
                    to: Some(Route::TopPage {})
                },
                BreadcrumbItem {
                    name: title.to_string(),
                    to: None,
                }
            ]
        }

        section {
            class: "my-5 py-5 px-3 bg-white",

            div {
                h1 {
                    class: "pb-8 text-3xl max-sm:text-2xl font-bold",
                    {title}
                }
            }

            div {
                class: "flex flex-col",

                // URL エンコード/デコードフォーム
                div {
                    class: "mb-6",

                    h2 {
                        class: "text-xl font-bold mb-3",
                        "URL エンコード/デコード"
                    }

                    p {
                        class: "mb-4 text-gray-600",
                        "いずれかの入力欄に値を入力すると、自動的に変換されます。http://やhttps://のようなスキーマ部分はエンコードされません。"
                    }

                    // 入力フィールド
                    div {
                        class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-4",

                        // 通常テキスト入力
                        div {
                            class: "p-4 border border-gray-300 rounded-md",

                            label {
                                class: "block mb-2 font-medium",
                                "通常テキスト"
                            }

                            textarea {
                                class: "w-full p-2 border border-gray-300 rounded-md",
                                placeholder: "エンコードするテキストを入力",
                                rows: "4",
                                value: normal_text(),
                                oninput: on_normal_input,
                            }
                        }

                        // エンコード済みテキスト入力
                        div {
                            class: "p-4 border border-gray-300 rounded-md",

                            label {
                                class: "block mb-2 font-medium",
                                "エンコード済みテキスト"
                            }

                            textarea {
                                class: "w-full p-2 border border-gray-300 rounded-md",
                                placeholder: "デコードするテキストを入力",
                                rows: "4",
                                value: encoded_text(),
                                oninput: on_encoded_input,
                            }
                        }
                    }

                    // エラー表示
                    if !error_message().is_empty() {
                        div {
                            class: "mt-4 p-3 bg-red-100 border border-red-300 rounded-md text-red-700",
                            {error_message()}
                        }
                    }
                }

                // 使い方説明
                Usage {
                    sections: vec![
                        UsageSectionProps {
                            title: "基本的な使い方".to_string(),
                            items: vec![
                                "左側の「通常テキスト」欄に文字を入力すると、右側の「エンコード済みテキスト」欄に自動的にエンコードされた結果が表示されます。".to_string(),
                                "右側の「エンコード済みテキスト」欄に文字を入力すると、左側の「通常テキスト」欄に自動的にデコードされた結果が表示されます。".to_string(),
                                "http://やhttps://のようなスキーマ部分はエンコードされずにそのまま表示されます。".to_string(),
                            ],
                        },
                        UsageSectionProps {
                            title: "URL エンコードとは".to_string(),
                            items: vec![
                                "URL エンコードは、URL で使用できない文字（日本語や特殊文字など）を URL で使用できる形式に変換することです。".to_string(),
                                "例えば、スペースは「%20」に、日本語は「%E3%81%82」のような形式に変換されます。".to_string(),
                            ],
                        },
                    ],
                }
            }
        }
    }
}
