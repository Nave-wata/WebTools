use crate::components::breadcrumb::{BreadcrumbItem, BreadcrumbList};
use crate::components::head::Head;
use crate::components::instructions::usage::{Usage, UsageSectionProps};
use crate::routes::Route;
use dioxus::prelude::*;
use serde::Serialize;
use serde_json::Value;

/// JSON フォーマッターツールページコンポーネント
///
/// このコンポーネントは以下の機能を提供します：
/// * JSON の整形（フォーマット）
/// * JSON の圧縮（ミニファイ）
/// * JSON の構文検証
/// * インデントタイプの設定（タブ、スペース）
/// * インデントサイズの設定
/// * コピー機能
pub(crate) fn JsonFormatter() -> Element {
    let title: &str = "JSON フォーマッター";
    let description: &str = "JSON データを整形・圧縮するツールです。リアルタイムで JSON の構文チェックを行い、適切なインデントと改行を追加してコードを読みやすくしたり、不要な空白を削除して圧縮したりできます。";

    // 入力JSON
    let mut input_json = use_signal(String::new);

    // 整形済みJSON
    let mut formatted_json = use_signal(String::new);

    // 圧縮済みJSON
    let mut compressed_json = use_signal(String::new);

    // 最後に編集されたフィールド
    let mut last_edited = use_signal(|| "");

    // エラーメッセージ
    let mut error_message = use_signal(String::new);

    // インデント設定
    let mut indent_type = use_signal(|| "space".to_string());
    let mut indent_size = use_signal(|| 2);

    // コピー成功メッセージ
    let mut copy_success = use_signal(String::new);

    // JSON処理関数
    let process_json = move |json_str: &str| -> (String, String, String) {
        if json_str.trim().is_empty() {
            return (String::new(), String::new(), String::new());
        }

        match serde_json::from_str::<Value>(json_str) {
            Ok(value) => {
                // 圧縮JSON
                let compressed = serde_json::to_string(&value).unwrap_or_default();

                // 整形JSON
                let formatted = if indent_type() == "tab" {
                    // タブインデントの場合
                    let tabs = vec![b'\t'; 1];
                    let formatter = serde_json::ser::PrettyFormatter::with_indent(tabs.as_slice());
                    let mut ser = serde_json::Serializer::with_formatter(Vec::new(), formatter);
                    value.serialize(&mut ser).unwrap();
                    String::from_utf8(ser.into_inner()).unwrap_or_default()
                } else {
                    // スペースインデントの場合
                    let spaces = vec![b' '; indent_size() as usize];
                    let formatter =
                        serde_json::ser::PrettyFormatter::with_indent(spaces.as_slice());
                    let mut ser = serde_json::Serializer::with_formatter(Vec::new(), formatter);
                    value.serialize(&mut ser).unwrap();
                    String::from_utf8(ser.into_inner()).unwrap_or_default()
                };

                (String::new(), formatted, compressed)
            }
            Err(e) => (
                format!("JSON構文エラー: {e}"),
                String::new(),
                String::new(),
            ),
        }
    };

    // 入力JSON変更処理
    let on_input_change = move |e: Event<FormData>| {
        let value = e.value().to_string();
        input_json.set(value.clone());
        last_edited.set("input");
        copy_success.set(String::new());

        let (error, formatted, compressed) = process_json(&value);
        error_message.set(error);
        formatted_json.set(formatted);
        compressed_json.set(compressed);
    };

    // 整形JSON変更処理
    let on_formatted_change = move |e: Event<FormData>| {
        let value = e.value().to_string();
        formatted_json.set(value.clone());
        last_edited.set("formatted");
        copy_success.set(String::new());

        let (error, _, compressed) = process_json(&value);
        let is_empty = error.is_empty();
        error_message.set(error);
        if is_empty {
            input_json.set(value);
            compressed_json.set(compressed);
        }
    };

    // 圧縮JSON変更処理
    let on_compressed_change = move |e: Event<FormData>| {
        let value = e.value().to_string();
        compressed_json.set(value.clone());
        last_edited.set("compressed");
        copy_success.set(String::new());

        let (error, formatted, _) = process_json(&value);
        let is_empty = error.is_empty();
        error_message.set(error);
        if is_empty {
            input_json.set(value);
            formatted_json.set(formatted);
        }
    };

    // インデントタイプ変更処理
    let on_indent_type_change = move |e: Event<FormData>| {
        let value = e.value().to_string();
        indent_type.set(value);
        // 設定変更時に整形し直す
        if !input_json().is_empty() {
            let (error, formatted, compressed) = process_json(&input_json());
            error_message.set(error);
            formatted_json.set(formatted);
            compressed_json.set(compressed);
        }
    };

    // インデントサイズ変更処理
    let on_indent_size_change = move |e: Event<FormData>| {
        if let Ok(size) = e.value().parse::<usize>() {
            if (1..=8).contains(&size) {
                indent_size.set(size);
                // 設定変更時に整形し直す
                if !input_json().is_empty() {
                    let (error, formatted, compressed) = process_json(&input_json());
                    error_message.set(error);
                    formatted_json.set(formatted);
                    compressed_json.set(compressed);
                }
            }
        }
    };

    // コピー機能
    let copy_to_clipboard = move |text: String, field_name: &str| {
        let field_name = field_name.to_string();
        spawn(async move {
            if let Some(window) = web_sys::window() {
                let navigator = window.navigator();
                let clipboard = navigator.clipboard();
                let promise = clipboard.write_text(&text);
                match wasm_bindgen_futures::JsFuture::from(promise).await {
                    Ok(_) => {
                        copy_success.set(format!("{field_name}をクリップボードにコピーしました"))
                    }
                    Err(_) => copy_success.set("コピーに失敗しました".to_string()),
                }
            }
        });
    };

    rsx! {
        Head {
            title: title,
            description: description,
            og_url: Route::JsonFormatter {}.to_string(),
            og_image: asset!("/assets/images/ogp/json_formatter.webp"),
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

                // JSON フォーマッター
                div {
                    class: "mb-6",

                    h2 {
                        class: "text-xl font-bold mb-3",
                        "JSON フォーマッター"
                    }

                    p {
                        class: "mb-4 text-gray-600",
                        "いずれかの入力欄にJSONを入力すると、自動的に整形・圧縮されたJSONが表示されます。"
                    }

                    // インデント設定
                    div {
                        class: "mb-4 p-4 bg-gray-50 rounded-md",

                        h3 {
                            class: "text-lg font-bold mb-2",
                            "インデント設定"
                        }

                        div {
                            class: "flex flex-wrap gap-4",

                            div {
                                class: "flex items-center gap-2",

                                label {
                                    class: "font-medium",
                                    "インデントタイプ:"
                                }

                                select {
                                    class: "p-2 border border-gray-300 rounded-md",
                                    value: indent_type(),
                                    onchange: on_indent_type_change,

                                    option {
                                        value: "space",
                                        "スペース"
                                    }
                                    option {
                                        value: "tab",
                                        "タブ"
                                    }
                                }
                            }

                            if indent_type() == "space" {
                                div {
                                    class: "flex items-center gap-2",

                                    label {
                                        class: "font-medium",
                                        "スペース数:"
                                    }

                                    input {
                                        class: "p-2 border border-gray-300 rounded-md w-20",
                                        r#type: "number",
                                        min: "1",
                                        max: "8",
                                        value: indent_size(),
                                        oninput: on_indent_size_change,
                                    }
                                }
                            }
                        }
                    }

                    // 入力フィールド
                    div {
                        class: "space-y-4",

                        // 入力JSON
                        div {
                            class: "p-4 border border-gray-300 rounded-md",

                            div {
                                class: "flex items-center justify-between mb-2",

                                label {
                                    class: "font-medium",
                                    "入力JSON"
                                }

                                button {
                                    class: "px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:opacity-50",
                                    disabled: input_json().is_empty(),
                                    onclick: move |_| copy_to_clipboard(input_json(), "入力JSON"),
                                    "コピー"
                                }
                            }

                            textarea {
                                class: "w-full p-2 border border-gray-300 rounded-md font-mono",
                                placeholder: "JSONを入力してください",
                                rows: "6",
                                value: input_json(),
                                oninput: on_input_change,
                            }
                        }

                        // 整形済みJSON
                        div {
                            class: "p-4 border border-gray-300 rounded-md",

                            div {
                                class: "flex items-center justify-between mb-2",

                                label {
                                    class: "font-medium",
                                    "整形済みJSON"
                                }

                                button {
                                    class: "px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:opacity-50",
                                    disabled: formatted_json().is_empty(),
                                    onclick: move |_| copy_to_clipboard(formatted_json(), "整形済みJSON"),
                                    "コピー"
                                }
                            }

                            textarea {
                                class: "w-full p-2 border border-gray-300 rounded-md font-mono",
                                placeholder: "整形済みJSONが表示されます",
                                rows: "6",
                                value: formatted_json(),
                                oninput: on_formatted_change,
                            }
                        }

                        // 圧縮済みJSON
                        div {
                            class: "p-4 border border-gray-300 rounded-md",

                            div {
                                class: "flex items-center justify-between mb-2",

                                label {
                                    class: "font-medium",
                                    "圧縮済みJSON"
                                }

                                button {
                                    class: "px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:opacity-50",
                                    disabled: compressed_json().is_empty(),
                                    onclick: move |_| copy_to_clipboard(compressed_json(), "圧縮済みJSON"),
                                    "コピー"
                                }
                            }

                            textarea {
                                class: "w-full p-2 border border-gray-300 rounded-md font-mono",
                                placeholder: "圧縮済みJSONが表示されます",
                                rows: "3",
                                value: compressed_json(),
                                oninput: on_compressed_change,
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

                    // コピー成功メッセージ
                    if !copy_success().is_empty() {
                        div {
                            class: "mt-4 p-3 bg-green-100 border border-green-300 rounded-md text-green-700",
                            {copy_success()}
                        }
                    }
                }

                // 使い方説明
                Usage {
                    sections: vec![
                        UsageSectionProps {
                            title: "基本的な使い方".to_string(),
                            items: vec![
                                "「入力JSON」欄にJSONデータを入力すると、自動的に整形済みと圧縮済みのJSONが表示されます。".to_string(),
                                "いずれかの入力欄を編集すると、他の欄も自動的に更新されます。".to_string(),
                                "各欄の「コピー」ボタンをクリックすると、そのJSONをクリップボードにコピーできます。".to_string(),
                            ],
                        },
                        UsageSectionProps {
                            title: "インデント設定".to_string(),
                            items: vec![
                                "インデントタイプを「タブ」または「スペース」から選択できます。".to_string(),
                                "スペースを選択した場合、1〜8個のスペース数を設定できます。".to_string(),
                                "設定を変更すると、整形済みJSONが自動的に更新されます。".to_string(),
                            ],
                        },
                        UsageSectionProps {
                            title: "機能".to_string(),
                            items: vec![
                                "整形: JSONにインデントと改行を追加して読みやすくします。".to_string(),
                                "圧縮: 不要な空白や改行を削除してJSONを最小化します。".to_string(),
                                "構文チェック: JSONの構文エラーを検出して表示します。".to_string(),
                                "リアルタイム変換: 入力と同時に結果が表示されます。".to_string(),
                            ],
                        },
                        UsageSectionProps {
                            title: "使用例".to_string(),
                            items: vec![
                                "APIレスポンスの整形・読みやすさの向上".to_string(),
                                "設定ファイルの整理とフォーマット統一".to_string(),
                                "JSONデータの構文チェックとデバッグ".to_string(),
                                "データの圧縮とファイルサイズ削減".to_string(),
                            ],
                        },
                    ],
                }
            }
        }
    }
}
