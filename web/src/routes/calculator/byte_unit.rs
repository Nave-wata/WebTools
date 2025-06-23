use crate::components::breadcrumb::{BreadcrumbItem, BreadcrumbList};
use crate::components::head::Head;
use crate::components::instructions::usage::{Usage, UsageSectionProps};
use crate::libs::calculator::byte_unit::{bytes_to_unit, unit_to_bytes};
use crate::routes::Route;
use dioxus::prelude::*;

/// 単位変換用の入力Signal群
struct UnitInputs<'a> {
    b_input: &'a mut Signal<String>,
    kb_input: &'a mut Signal<String>,
    mb_input: &'a mut Signal<String>,
    gb_input: &'a mut Signal<String>,
    tb_input: &'a mut Signal<String>,
}

/// バイト単位変換と共通エラー処理を行うヘルパー関数
///
/// # Arguments
/// * `convert_fn` - 変換関数
/// * `value` - 変換する値
/// * `error_message` - エラーメッセージを格納するSignal
///
/// # Returns
/// * `Ok(String)` - 変換に成功した場合、変換後の値
/// * `Err(())` - 変換に失敗した場合
fn convert_and_handle_error<F>(
    convert_fn: F,
    value: &str,
    error_message: &mut Signal<String>,
) -> Result<String, ()>
where
    F: Fn(&str) -> Result<String, String>,
{
    match convert_fn(value) {
        Ok(result) => Ok(result),
        Err(err) => {
            error_message.set(err);
            Err(())
        }
    }
}

/// 入力値を更新し、最後に編集されたフィールドを設定する
///
/// # Arguments
/// * `input_unit` - 入力単位 ("B", "KB", "MB", "GB", "TB")
/// * `value` - 入力された値
/// * `inputs` - 各単位の入力値のSignal
/// * `last_edited` - 最後に編集されたフィールドを追跡するSignal
fn update_input_value(
    input_unit: &str,
    value: &str,
    inputs: UnitInputs,
    last_edited: &mut Signal<&str>,
) {
    // 入力値を更新
    match input_unit {
        "B" => inputs.b_input.set(value.to_string()),
        "KB" => inputs.kb_input.set(value.to_string()),
        "MB" => inputs.mb_input.set(value.to_string()),
        "GB" => inputs.gb_input.set(value.to_string()),
        "TB" => inputs.tb_input.set(value.to_string()),
        _ => panic!("Unknown input unit"),
    }

    // 最後に編集されたフィールドを更新
    match input_unit {
        "B" => last_edited.set("B"),
        "KB" => last_edited.set("KB"),
        "MB" => last_edited.set("MB"),
        "GB" => last_edited.set("GB"),
        "TB" => last_edited.set("TB"),
        _ => panic!("Unknown input unit"),
    }
}

/// 空の入力の場合、他の入力フィールドをクリアする
///
/// # Arguments
/// * `input_unit` - 入力単位 ("B", "KB", "MB", "GB", "TB")
/// * `inputs` - 各単位の入力値のSignal
fn clear_other_inputs(input_unit: &str, inputs: UnitInputs) {
    match input_unit {
        "B" => {
            inputs.kb_input.set(String::new());
            inputs.mb_input.set(String::new());
            inputs.gb_input.set(String::new());
            inputs.tb_input.set(String::new());
        }
        "KB" => {
            inputs.b_input.set(String::new());
            inputs.mb_input.set(String::new());
            inputs.gb_input.set(String::new());
            inputs.tb_input.set(String::new());
        }
        "MB" => {
            inputs.b_input.set(String::new());
            inputs.kb_input.set(String::new());
            inputs.gb_input.set(String::new());
            inputs.tb_input.set(String::new());
        }
        "GB" => {
            inputs.b_input.set(String::new());
            inputs.kb_input.set(String::new());
            inputs.mb_input.set(String::new());
            inputs.tb_input.set(String::new());
        }
        "TB" => {
            inputs.b_input.set(String::new());
            inputs.kb_input.set(String::new());
            inputs.mb_input.set(String::new());
            inputs.gb_input.set(String::new());
        }
        _ => panic!("Unknown input unit"),
    }
}

/// 入力単位に応じた変換処理を行う
///
/// # Arguments
/// * `input_unit` - 入力単位 ("B", "KB", "MB", "GB", "TB")
/// * `value` - 入力された値
/// * `inputs` - 各単位の入力値のSignal
/// * `error_message` - エラーメッセージを格納するSignal
fn convert_units(
    input_unit: &str,
    value: &str,
    inputs: UnitInputs,
    error_message: &mut Signal<String>,
) {
    match input_unit {
        "B" => {
            // Bから他の単位への変換
            let target_units = [
                ("KB", inputs.kb_input),
                ("MB", inputs.mb_input),
                ("GB", inputs.gb_input),
                ("TB", inputs.tb_input),
            ];

            for (unit, input_signal) in target_units {
                if let Ok(converted) =
                    convert_and_handle_error::<_>(|v| bytes_to_unit(v, unit), value, error_message)
                {
                    input_signal.set(converted);
                }
            }
        }
        "KB" => {
            // KBからBへの変換
            if let Ok(b) =
                convert_and_handle_error::<_>(|v| unit_to_bytes(v, "KB"), value, error_message)
            {
                inputs.b_input.set(b.clone());

                // Bから他の単位への変換
                if let Ok(mb) =
                    convert_and_handle_error::<_>(|v| bytes_to_unit(v, "MB"), &b, error_message)
                {
                    inputs.mb_input.set(mb);
                }
                if let Ok(gb) =
                    convert_and_handle_error::<_>(|v| bytes_to_unit(v, "GB"), &b, error_message)
                {
                    inputs.gb_input.set(gb);
                }
                if let Ok(tb) =
                    convert_and_handle_error::<_>(|v| bytes_to_unit(v, "TB"), &b, error_message)
                {
                    inputs.tb_input.set(tb);
                }
            }
        }
        "MB" => {
            // MBからBへの変換
            if let Ok(b) =
                convert_and_handle_error::<_>(|v| unit_to_bytes(v, "MB"), value, error_message)
            {
                inputs.b_input.set(b.clone());

                // Bから他の単位への変換
                if let Ok(kb) =
                    convert_and_handle_error::<_>(|v| bytes_to_unit(v, "KB"), &b, error_message)
                {
                    inputs.kb_input.set(kb);
                }
                if let Ok(gb) =
                    convert_and_handle_error::<_>(|v| bytes_to_unit(v, "GB"), &b, error_message)
                {
                    inputs.gb_input.set(gb);
                }
                if let Ok(tb) =
                    convert_and_handle_error::<_>(|v| bytes_to_unit(v, "TB"), &b, error_message)
                {
                    inputs.tb_input.set(tb);
                }
            }
        }
        "GB" => {
            // GBからBへの変換
            if let Ok(b) =
                convert_and_handle_error::<_>(|v| unit_to_bytes(v, "GB"), value, error_message)
            {
                inputs.b_input.set(b.clone());

                // Bから他の単位への変換
                if let Ok(kb) =
                    convert_and_handle_error::<_>(|v| bytes_to_unit(v, "KB"), &b, error_message)
                {
                    inputs.kb_input.set(kb);
                }
                if let Ok(mb) =
                    convert_and_handle_error::<_>(|v| bytes_to_unit(v, "MB"), &b, error_message)
                {
                    inputs.mb_input.set(mb);
                }
                if let Ok(tb) =
                    convert_and_handle_error::<_>(|v| bytes_to_unit(v, "TB"), &b, error_message)
                {
                    inputs.tb_input.set(tb);
                }
            }
        }
        "TB" => {
            // TBからBへの変換
            if let Ok(b) =
                convert_and_handle_error::<_>(|v| unit_to_bytes(v, "TB"), value, error_message)
            {
                inputs.b_input.set(b.clone());

                // Bから他の単位への変換
                if let Ok(kb) =
                    convert_and_handle_error::<_>(|v| bytes_to_unit(v, "KB"), &b, error_message)
                {
                    inputs.kb_input.set(kb);
                }
                if let Ok(mb) =
                    convert_and_handle_error::<_>(|v| bytes_to_unit(v, "MB"), &b, error_message)
                {
                    inputs.mb_input.set(mb);
                }
                if let Ok(gb) =
                    convert_and_handle_error::<_>(|v| bytes_to_unit(v, "GB"), &b, error_message)
                {
                    inputs.gb_input.set(gb);
                }
            }
        }
        _ => panic!("Unknown input unit"),
    }
}

/// 入力処理を共通化するヘルパー関数
///
/// # Arguments
/// * `input_unit` - 入力単位 ("B", "KB", "MB", "GB", "TB")
/// * `value` - 入力された値
/// * `inputs` - 各単位の入力値のSignal
/// * `last_edited` - 最後に編集されたフィールドを追跡するSignal
/// * `error_message` - エラーメッセージを格納するSignal
fn handle_input(
    input_unit: &str,
    value: String,
    inputs: UnitInputs,
    last_edited: &mut Signal<&str>,
    error_message: &mut Signal<String>,
) {
    // 入力値と最後に編集されたフィールドを更新
    update_input_value(
        input_unit,
        &value,
        UnitInputs {
            b_input: inputs.b_input,
            kb_input: inputs.kb_input,
            mb_input: inputs.mb_input,
            gb_input: inputs.gb_input,
            tb_input: inputs.tb_input,
        },
        last_edited,
    );

    error_message.set(String::new());

    // 空の入力の場合、他の入力フィールドをクリア
    if value.is_empty() {
        clear_other_inputs(
            input_unit,
            UnitInputs {
                b_input: inputs.b_input,
                kb_input: inputs.kb_input,
                mb_input: inputs.mb_input,
                gb_input: inputs.gb_input,
                tb_input: inputs.tb_input,
            },
        );
        return;
    }

    // 入力単位に応じた変換処理
    convert_units(input_unit, &value, inputs, error_message);
}

/// バイト単位変換ツールページコンポーネント
///
/// このコンポーネントは以下の機能を提供します：
/// * B、KB、MB、GB、TBの相互変換
pub(crate) fn ByteUnitCalculator() -> Element {
    let title: &str = "バイト単位変換";
    let description: &str = "入力された値を異なるバイト単位（B、KB、MB、GB、TB）に相互変換するツールです。これらの変換は入力された値を元に、リアルタイムで他の単位の表現に変換することが可能です。";

    // 各単位の入力値
    let mut b_input = use_signal(String::new);
    let mut kb_input = use_signal(String::new);
    let mut mb_input = use_signal(String::new);
    let mut gb_input = use_signal(String::new);
    let mut tb_input = use_signal(String::new);

    // エラーメッセージ
    let mut error_message = use_signal(String::new);

    // 入力元の単位を追跡（最後に編集されたフィールド）
    let mut last_edited = use_signal(|| "");

    // B入力時の処理
    let on_b_input = move |e: Event<FormData>| {
        let value = e.value().to_string();
        handle_input(
            "B",
            value,
            UnitInputs {
                b_input: &mut b_input,
                kb_input: &mut kb_input,
                mb_input: &mut mb_input,
                gb_input: &mut gb_input,
                tb_input: &mut tb_input,
            },
            &mut last_edited,
            &mut error_message,
        );
    };

    // KB入力時の処理
    let on_kb_input = move |e: Event<FormData>| {
        let value = e.value().to_string();
        handle_input(
            "KB",
            value,
            UnitInputs {
                b_input: &mut b_input,
                kb_input: &mut kb_input,
                mb_input: &mut mb_input,
                gb_input: &mut gb_input,
                tb_input: &mut tb_input,
            },
            &mut last_edited,
            &mut error_message,
        );
    };

    // MB入力時の処理
    let on_mb_input = move |e: Event<FormData>| {
        let value = e.value().to_string();
        handle_input(
            "MB",
            value,
            UnitInputs {
                b_input: &mut b_input,
                kb_input: &mut kb_input,
                mb_input: &mut mb_input,
                gb_input: &mut gb_input,
                tb_input: &mut tb_input,
            },
            &mut last_edited,
            &mut error_message,
        );
    };

    // GB入力時の処理
    let on_gb_input = move |e: Event<FormData>| {
        let value = e.value().to_string();
        handle_input(
            "GB",
            value,
            UnitInputs {
                b_input: &mut b_input,
                kb_input: &mut kb_input,
                mb_input: &mut mb_input,
                gb_input: &mut gb_input,
                tb_input: &mut tb_input,
            },
            &mut last_edited,
            &mut error_message,
        );
    };

    // TB入力時の処理
    let on_tb_input = move |e: Event<FormData>| {
        let value = e.value().to_string();
        handle_input(
            "TB",
            value,
            UnitInputs {
                b_input: &mut b_input,
                kb_input: &mut kb_input,
                mb_input: &mut mb_input,
                gb_input: &mut gb_input,
                tb_input: &mut tb_input,
            },
            &mut last_edited,
            &mut error_message,
        );
    };

    rsx! {
        Head {
            title: title,
            description: description,
            og_url: Route::ByteUnitCalculator {}.to_string(),
            og_image: asset!("/assets/images/ogp/byte_unit_calculator.webp"),
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

                // バイト単位変換フォーム
                div {
                    class: "mb-6",

                    h2 {
                        class: "text-xl font-bold mb-3",
                        "バイト単位変換"
                    }

                    p {
                        class: "mb-4 text-gray-600",
                        "いずれかの入力欄に値を入力すると、自動的に他の単位に変換されます。"
                    }

                    // 入力フィールド
                    div {
                        class: "grid grid-cols-1 md:grid-cols-3 gap-4 mb-4",

                        // B入力
                        div {
                            class: "p-4 border border-gray-300 rounded-md",

                            label {
                                class: "block mb-2 font-medium",
                                "バイト (B)"
                            }

                            input {
                                class: "w-full p-2 border border-gray-300 rounded-md",
                                placeholder: "バイト数を入力",
                                value: b_input(),
                                oninput: on_b_input,
                            }
                        }

                        // KB入力
                        div {
                            class: "p-4 border border-gray-300 rounded-md",

                            label {
                                class: "block mb-2 font-medium",
                                "キロバイト (KB)"
                            }

                            input {
                                class: "w-full p-2 border border-gray-300 rounded-md",
                                placeholder: "キロバイト数を入力",
                                value: kb_input(),
                                oninput: on_kb_input,
                            }
                        }

                        // MB入力
                        div {
                            class: "p-4 border border-gray-300 rounded-md",

                            label {
                                class: "block mb-2 font-medium",
                                "メガバイト (MB)"
                            }

                            input {
                                class: "w-full p-2 border border-gray-300 rounded-md",
                                placeholder: "メガバイト数を入力",
                                value: mb_input(),
                                oninput: on_mb_input,
                            }
                        }

                        // GB入力
                        div {
                            class: "p-4 border border-gray-300 rounded-md",

                            label {
                                class: "block mb-2 font-medium",
                                "ギガバイト (GB)"
                            }

                            input {
                                class: "w-full p-2 border border-gray-300 rounded-md",
                                placeholder: "ギガバイト数を入力",
                                value: gb_input(),
                                oninput: on_gb_input,
                            }
                        }

                        // TB入力
                        div {
                            class: "p-4 border border-gray-300 rounded-md",

                            label {
                                class: "block mb-2 font-medium",
                                "テラバイト (TB)"
                            }

                            input {
                                class: "w-full p-2 border border-gray-300 rounded-md",
                                placeholder: "テラバイト数を入力",
                                value: tb_input(),
                                oninput: on_tb_input,
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
                                "各入力欄には数値を入力できます。".to_string(),
                                "いずれかの入力欄に値を入力すると、自動的に他の単位に変換されます。".to_string(),
                                "変換は1KB = 1024B、1MB = 1024KB、1GB = 1024MB、1TB = 1024GBとして計算されます。".to_string(),
                            ],
                        },
                    ],
                }
            }
        }
    }
}
