use crate::components::breadcrumb::{BreadcrumbItem, BreadcrumbList};
use crate::components::head::Head;
use crate::components::instructions::usage::{Usage, UsageSectionProps};
use crate::libs::base::{
    base16_decode, base16_encode, base32_decode, base32_encode, base64_decode, base64_encode,
};
use crate::routes::Route;
use dioxus::prelude::*;

/// Base エンコード/デコードツールページコンポーネント
///
/// このコンポーネントは以下の機能を提供します：
/// * Base16 (Hex) のエンコードとデコード
/// * Base32 のエンコードとデコード
/// * Base64 のエンコードとデコード
pub(crate) fn BaseEnDecoder() -> Element {
    let title: &str = "Base エンコード/デコード";
    let description: &str = "Base16, Base32, Base64 のエンコードとデコードを行うツールです。テキストを各種Base形式でエンコードしたり、エンコードされたデータをデコードしたりすることができます。";

    // 通常テキスト
    let mut normal_text = use_signal(String::new);

    // エンコード済みテキスト
    let mut base16_text = use_signal(String::new);
    let mut base32_text = use_signal(String::new);
    let mut base64_text = use_signal(String::new);

    // 最後に編集されたフィールド
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
            base16_text.set(String::new());
            base32_text.set(String::new());
            base64_text.set(String::new());
            return;
        }

        // リアルタイムでエンコード
        base16_text.set(base16_encode(&value));
        base32_text.set(base32_encode(&value));
        base64_text.set(base64_encode(&value));
    };

    // Base16 入力処理
    let on_base16_input = move |e: Event<FormData>| {
        let value = e.value().to_string();
        base16_text.set(value.clone());
        last_edited.set("base16");
        error_message.set(String::new());

        // 入力が空の場合は他のテキストもクリア
        if value.is_empty() {
            normal_text.set(String::new());
            base32_text.set(String::new());
            base64_text.set(String::new());
            return;
        }

        // リアルタイムでデコード
        match base16_decode(&value) {
            Ok(result) => {
                normal_text.set(result.clone());
                base32_text.set(base32_encode(&result));
                base64_text.set(base64_encode(&result));
            }
            Err(err) => error_message.set(err),
        }
    };

    // Base32 入力処理
    let on_base32_input = move |e: Event<FormData>| {
        let value = e.value().to_string();
        base32_text.set(value.clone());
        last_edited.set("base32");
        error_message.set(String::new());

        // 入力が空の場合は他のテキストもクリア
        if value.is_empty() {
            normal_text.set(String::new());
            base16_text.set(String::new());
            base64_text.set(String::new());
            return;
        }

        // リアルタイムでデコード
        match base32_decode(&value) {
            Ok(result) => {
                normal_text.set(result.clone());
                base16_text.set(base16_encode(&result));
                base64_text.set(base64_encode(&result));
            }
            Err(err) => error_message.set(err),
        }
    };

    // Base64 入力処理
    let on_base64_input = move |e: Event<FormData>| {
        let value = e.value().to_string();
        base64_text.set(value.clone());
        last_edited.set("base64");
        error_message.set(String::new());

        // 入力が空の場合は他のテキストもクリア
        if value.is_empty() {
            normal_text.set(String::new());
            base16_text.set(String::new());
            base32_text.set(String::new());
            return;
        }

        // リアルタイムでデコード
        match base64_decode(&value) {
            Ok(result) => {
                normal_text.set(result.clone());
                base16_text.set(base16_encode(&result));
                base32_text.set(base32_encode(&result));
            }
            Err(err) => error_message.set(err),
        }
    };

    rsx! {
        Head {
            title: title,
            description: description,
            og_url: Route::BaseEnDecoder {}.to_string(),
            og_image: asset!("/assets/images/ogp/base_endecode_converter.webp"),
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

                // Base エンコード/デコードフォーム
                div {
                    class: "mb-6",

                    h2 {
                        class: "text-xl font-bold mb-3",
                        "Base エンコード/デコード"
                    }

                    p {
                        class: "mb-4 text-gray-600",
                        "いずれかの入力欄に値を入力すると、自動的に他の形式に変換されます。"
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

                        // Base16 入力
                        div {
                            class: "p-4 border border-gray-300 rounded-md",

                            label {
                                class: "block mb-2 font-medium",
                                "Base16 (Hex)"
                            }

                            textarea {
                                class: "w-full p-2 border border-gray-300 rounded-md font-mono",
                                placeholder: "Base16でエンコードされたテキストを入力",
                                rows: "4",
                                value: base16_text(),
                                oninput: on_base16_input,
                            }
                        }

                        // Base32 入力
                        div {
                            class: "p-4 border border-gray-300 rounded-md",

                            label {
                                class: "block mb-2 font-medium",
                                "Base32"
                            }

                            textarea {
                                class: "w-full p-2 border border-gray-300 rounded-md font-mono",
                                placeholder: "Base32でエンコードされたテキストを入力",
                                rows: "4",
                                value: base32_text(),
                                oninput: on_base32_input,
                            }
                        }

                        // Base64 入力
                        div {
                            class: "p-4 border border-gray-300 rounded-md",

                            label {
                                class: "block mb-2 font-medium",
                                "Base64"
                            }

                            textarea {
                                class: "w-full p-2 border border-gray-300 rounded-md font-mono",
                                placeholder: "Base64でエンコードされたテキストを入力",
                                rows: "4",
                                value: base64_text(),
                                oninput: on_base64_input,
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
                                "「通常テキスト」欄に文字を入力すると、自動的にBase16、Base32、Base64の各形式でエンコードされた結果が表示されます。".to_string(),
                                "各Base形式の入力欄にエンコードされたテキストを入力すると、自動的にデコードされて通常テキストと他の形式が表示されます。".to_string(),
                                "リアルタイムで変換されるため、入力と同時に結果を確認できます。".to_string(),
                            ],
                        },
                        UsageSectionProps {
                            title: "各Base形式について".to_string(),
                            items: vec![
                                "Base16 (Hex): 16進数表記で、0-9とA-Fの文字を使用します。バイナリデータを人間が読める形式で表現するのに使われます。".to_string(),
                                "Base32: 32文字のアルファベット（A-Z、2-7）を使用します。URLやファイル名に安全に使用できる文字のみを使用します。".to_string(),
                                "Base64: 64文字（A-Z、a-z、0-9、+、/）を使用します。メールやWebで広く使用される標準的なエンコード方式です。".to_string(),
                            ],
                        },
                        UsageSectionProps {
                            title: "使用例".to_string(),
                            items: vec![
                                "バイナリファイルをテキスト形式で送信する際のエンコード".to_string(),
                                "URLに含めることができない文字のエンコード".to_string(),
                                "データの整合性確認のためのハッシュ値の表現".to_string(),
                                "APIキーやトークンの安全な表現".to_string(),
                            ],
                        },
                    ],
                }
            }
        }
    }
}
