use crate::components::breadcrumb::{BreadcrumbItem, BreadcrumbList};
use crate::components::head::Head;
use crate::libs::number;
use crate::routes::Route;
use dioxus::prelude::*;
use std::fmt::Display;

/// 進数変換ツールページコンポーネント
///
/// このコンポーネントは以下の機能を提供します：
/// * 2進数、10進数、16進数の相互変換

/// 進数変換と共通エラー処理を行うヘルパー関数
///
/// # Arguments
/// * `convert_fn` - 変換関数（base_to_decimalまたはdecimal_to_base）
/// * `value` - 変換する値
/// * `base` - 変換元または変換先の基数
/// * `error_message` - エラーメッセージを格納するSignal
///
/// # Returns
/// * `Ok(String)` - 変換に成功した場合、変換後の値
/// * `Err(())` - 変換に失敗した場合
fn convert_and_handle_error<F, T>(
    convert_fn: F,
    value: &str,
    base: u32,
    error_message: &mut Signal<String>,
) -> Result<String, ()>
where
    F: Fn(&str, u32) -> Result<String, String>,
    T: Display,
{
    match convert_fn(value, base) {
        Ok(result) => Ok(result),
        Err(err) => {
            error_message.set(err);
            Err(())
        }
    }
}

/// 入力処理を共通化するヘルパー関数
///
/// # Arguments
/// * `input_type` - 入力タイプ ("binary", "decimal", "hex")
/// * `value` - 入力された値
/// * `binary_input` - 2進数入力値のSignal
/// * `decimal_input` - 10進数入力値のSignal
/// * `hex_input` - 16進数入力値のSignal
/// * `last_edited` - 最後に編集されたフィールドを追跡するSignal
/// * `error_message` - エラーメッセージを格納するSignal
fn handle_input(
    input_type: &str,
    value: String,
    binary_input: &mut Signal<String>,
    decimal_input: &mut Signal<String>,
    hex_input: &mut Signal<String>,
    last_edited: &mut Signal<&str>,
    error_message: &mut Signal<String>,
) {
    // 入力値と最後に編集されたフィールドを更新
    match input_type {
        "binary" => binary_input.set(value.clone()),
        "decimal" => decimal_input.set(value.clone()),
        "hex" => hex_input.set(value.clone()),
        _ => panic!("Unknown input type"),
    }

    // Use static string literals for last_edited
    match input_type {
        "binary" => last_edited.set("binary"),
        "decimal" => last_edited.set("decimal"),
        "hex" => last_edited.set("hex"),
        _ => panic!("Unknown input type"),
    };
    error_message.set(String::new());

    // 空の入力の場合、他の入力フィールドをクリア
    if value.is_empty() {
        match input_type {
            "binary" => {
                decimal_input.set(String::new());
                hex_input.set(String::new());
            }
            "decimal" => {
                binary_input.set(String::new());
                hex_input.set(String::new());
            }
            "hex" => {
                binary_input.set(String::new());
                decimal_input.set(String::new());
            }
            _ => panic!("Unknown input type"),
        }
        return;
    }

    // 入力タイプに応じた変換処理
    match input_type {
        "binary" => {
            // 2進数から10進数への変換
            if let Ok(decimal) = convert_and_handle_error::<_, String>(
                number::base_to_decimal,
                &value,
                2,
                error_message,
            ) {
                decimal_input.set(decimal.clone());

                // 10進数から16進数への変換
                if let Ok(hex) = convert_and_handle_error::<_, String>(
                    number::decimal_to_base,
                    &decimal,
                    16,
                    error_message,
                ) {
                    hex_input.set(hex);
                }
            }
        }
        "decimal" => {
            // 10進数から2進数への変換
            if let Ok(binary) = convert_and_handle_error::<_, String>(
                number::decimal_to_base,
                &value,
                2,
                error_message,
            ) {
                binary_input.set(binary);
            }

            // 10進数から16進数への変換
            if let Ok(hex) = convert_and_handle_error::<_, String>(
                number::decimal_to_base,
                &value,
                16,
                error_message,
            ) {
                hex_input.set(hex);
            }
        }
        "hex" => {
            // 16進数から10進数への変換
            if let Ok(decimal) = convert_and_handle_error::<_, String>(
                number::base_to_decimal,
                &value,
                16,
                error_message,
            ) {
                decimal_input.set(decimal.clone());

                // 10進数から2進数への変換
                if let Ok(binary) = convert_and_handle_error::<_, String>(
                    number::decimal_to_base,
                    &decimal,
                    2,
                    error_message,
                ) {
                    binary_input.set(binary);
                }
            }
        }
        _ => panic!("Unknown input type"),
    }
}

pub(crate) fn NumberBaseConverter() -> Element {
    let title: &str = "進数変換";
    let description: &str = "入力された数値を相互に進数変換するツールです。2進数、10進数、16進数の相互変換に対応しています。これらの変換は入力された数値を元に、リアルタイムで残り2種類の表現に変換することが可能です。";

    // 各進数の入力値
    let mut binary_input = use_signal(String::new);
    let mut decimal_input = use_signal(String::new);
    let mut hex_input = use_signal(String::new);

    // エラーメッセージ
    let mut error_message = use_signal(String::new);

    // 入力元の進数を追跡（最後に編集されたフィールド）
    let mut last_edited = use_signal(|| "");

    // 2進数入力時の処理
    let on_binary_input = move |e: Event<FormData>| {
        let value = e.value().to_string();
        handle_input(
            "binary",
            value,
            &mut binary_input,
            &mut decimal_input,
            &mut hex_input,
            &mut last_edited,
            &mut error_message,
        );
    };

    // 10進数入力時の処理
    let on_decimal_input = move |e: Event<FormData>| {
        let value = e.value().to_string();
        handle_input(
            "decimal",
            value,
            &mut binary_input,
            &mut decimal_input,
            &mut hex_input,
            &mut last_edited,
            &mut error_message,
        );
    };

    // 16進数入力時の処理
    let on_hex_input = move |e: Event<FormData>| {
        let value = e.value().to_string();
        handle_input(
            "hex",
            value,
            &mut binary_input,
            &mut decimal_input,
            &mut hex_input,
            &mut last_edited,
            &mut error_message,
        );
    };

    rsx! {
        Head {
            title: title,
            description: description,
            og_url: Route::NumberBaseConverter {}.to_string(),
            og_image: asset!("/assets/images/ogp/number_base_converter.webp"),
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

                // 進数変換フォーム
                div {
                    class: "mb-6",

                    h2 {
                        class: "text-xl font-bold mb-3",
                        "進数変換"
                    }

                    p {
                        class: "mb-4 text-gray-600",
                        "いずれかの入力欄に値を入力すると、自動的に他の進数に変換されます。"
                    }

                    // 入力フィールド
                    div {
                        class: "grid grid-cols-1 md:grid-cols-3 gap-4 mb-4",

                        // 2進数入力
                        div {
                            class: "p-4 border border-gray-300 rounded-md",

                            label {
                                class: "block mb-2 font-medium",
                                "2進数"
                            }

                            input {
                                class: "w-full p-2 border border-gray-300 rounded-md",
                                placeholder: "2進数を入力",
                                value: binary_input(),
                                oninput: on_binary_input,
                            }
                        }

                        // 10進数入力
                        div {
                            class: "p-4 border border-gray-300 rounded-md",

                            label {
                                class: "block mb-2 font-medium",
                                "10進数"
                            }

                            input {
                                class: "w-full p-2 border border-gray-300 rounded-md",
                                placeholder: "10進数を入力",
                                value: decimal_input(),
                                oninput: on_decimal_input,
                            }
                        }

                        // 16進数入力
                        div {
                            class: "p-4 border border-gray-300 rounded-md",

                            label {
                                class: "block mb-2 font-medium",
                                "16進数"
                            }

                            input {
                                class: "w-full p-2 border border-gray-300 rounded-md",
                                placeholder: "16進数を入力",
                                value: hex_input(),
                                oninput: on_hex_input,
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
                div {
                    class: "mb-6",

                    h2 {
                        class: "text-xl font-bold mb-3",
                        "使い方"
                    }

                    ul {
                        class: "list-disc pl-5 space-y-2",

                        li {
                            "2進数入力欄には「0」と「1」のみ入力できます。"
                        }

                        li {
                            "10進数入力欄には「0」から「9」までの数字を入力できます。"
                        }

                        li {
                            "16進数入力欄には「0」から「9」までの数字と「A」から「F」までの文字（大文字・小文字どちらも可）を入力できます。"
                        }

                        li {
                            "いずれかの入力欄に値を入力すると、自動的に他の進数に変換されます。"
                        }
                    }
                }
            }
        }
    }
}
