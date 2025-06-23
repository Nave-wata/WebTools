use crate::components::breadcrumb::{BreadcrumbItem, BreadcrumbList};
use crate::components::head::Head;
use crate::components::instructions::usage::{Usage, UsageSectionProps};
use crate::libs::calculator::big_numbers;
use crate::routes::Route;
use dioxus::prelude::*;
use keyboard_types::Key;

/// 大きな数値電卓コンポーネント
pub(crate) fn BigNumbersCalculator() -> Element {
    let title: &str = "大きな数値の電卓";
    let description: &str = "非常に大きな数値での計算に対応した電卓です。数値を文字列として処理するため、通常の電卓では扱えないような大きな数値でも計算できます。";

    // 入力値と結果
    let mut first_number = use_signal(String::new);
    let mut second_number = use_signal(String::new);
    let mut operator = use_signal(|| String::from("+"));
    let mut result = use_signal(|| String::from("0"));
    let mut expression = use_signal(String::new);
    let mut error = use_signal(String::new);

    // 計算を実行する関数
    let mut execute_form_calculation = move || {
        // 入力値の検証
        if first_number().is_empty() {
            error.set("最初の数値を入力してください".to_string());
            return;
        }
        if second_number().is_empty() {
            error.set("2つ目の数値を入力してください".to_string());
            return;
        }

        // 演算子に基づいて計算
        let op = operator();
        let expr = format!("{}{}{}", first_number(), op, second_number());
        let formatted_expr = format!("{} {} {}", first_number(), op, second_number());

        // 式を保存
        expression.set(formatted_expr);

        // 計算実行
        match big_numbers::evaluate_expression(&expr) {
            Ok(value) => {
                result.set(value);
                error.set(String::new());
            }
            Err(err) => {
                error.set(err);
            }
        }
    };

    // 入力フィールドの変更ハンドラ
    let on_first_number_change = move |evt: Event<FormData>| {
        first_number.set(evt.value().clone());
        error.set(String::new());
    };

    let on_second_number_change = move |evt: Event<FormData>| {
        second_number.set(evt.value().clone());
        error.set(String::new());
    };

    let on_operator_change = move |evt: Event<FormData>| {
        operator.set(evt.value().clone());
        error.set(String::new());
    };

    // キーボードイベントハンドラ
    let on_keydown = move |e: Event<KeyboardData>| {
        if e.key() == Key::Enter {
            execute_form_calculation();
        }
    };

    rsx! {
        Head {
            title: title,
            description: description,
            og_url: Route::BigNumbersCalculator {}.to_string(),
            og_image: asset!("/assets/images/ogp/big_numbers_calculator.webp"),
        }

        BreadcrumbList {
            items: vec! [
                BreadcrumbItem {
                    name: "トップ".to_string(),
                    to: Some(Route::TopPage {})
                },
                BreadcrumbItem {
                    name: title.to_string(),
                    to: None
                }
            ]
        }

        div {
            class: "container mx-auto px-4 py-8",
            h1 {
                class: "text-3xl font-bold mb-6 text-center",
                "{title}"
            }

            // 電卓本体（フォーム形式）
            div {
                class: "max-w-5xl mx-auto bg-white rounded-lg shadow-lg p-6",
                tabindex: 0,
                onkeydown: on_keydown,

                // 1つ目の数値入力
                div {
                    class: "mb-4",
                    label {
                        class: "block text-gray-700 text-sm font-bold mb-2",
                        "1つ目の数値"
                    }
                    input {
                        class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                        r#type: "text",
                        placeholder: "最初の数値を入力",
                        value: first_number,
                        oninput: on_first_number_change
                    }
                }

                // 演算子選択
                div {
                    class: "mb-4",
                    label {
                        class: "block text-gray-700 text-sm font-bold mb-2",
                        "演算子"
                    }
                    select {
                        class: "shadow border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                        value: operator,
                        onchange: on_operator_change,
                        option {
                            value: "+",
                            "+"
                        }
                        option {
                            value: "-",
                            "-"
                        }
                        option {
                            value: "×",
                            "×"
                        }
                        option {
                            value: "÷",
                            "÷"
                        }
                    }
                }

                // 2つ目の数値入力
                div {
                    class: "mb-4",
                    label {
                        class: "block text-gray-700 text-sm font-bold mb-2",
                        "2つ目の数値"
                    }
                    input {
                        class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                        r#type: "text",
                        placeholder: "2つ目の数値を入力",
                        value: second_number,
                        oninput: on_second_number_change
                    }
                }

                // 計算ボタン
                div {
                    class: "mb-4",
                    button {
                        class: "bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline w-full",
                        onclick: move |_| execute_form_calculation(),
                        "計算する"
                    }
                }

                // 結果表示
                div {
                    class: "mb-4",
                    label {
                        class: "block text-gray-700 text-sm font-bold mb-2",
                        "計算結果"
                    }
                    div {
                        class: "bg-gray-100 p-4 rounded-lg",
                        if !error().is_empty() {
                            span {
                                class: "text-red-500 font-bold",
                                "{error}"
                            }
                        } else {
                            // 計算式の表示
                            if !expression().is_empty() {
                                div {
                                    class: "text-gray-600 text-sm mb-2 break-all",
                                    "{expression}"
                                }
                            }
                            // 結果の表示（より目立つスタイル）
                            div {
                                class: "text-2xl font-bold text-green-700 break-all",
                                "{result}"
                            }
                        }
                    }
                }

                // 使い方説明
                Usage {
                    sections: vec![
                        UsageSectionProps {
                            title: "基本的な使い方".to_string(),
                            items: vec![
                                "「最初の数値」欄に計算したい最初の数値を入力します。".to_string(),
                                "「演算子」ドロップダウンから計算に使用する演算子（+, -, ×, ÷）を選択します。".to_string(),
                                "「2つ目の数値」欄に計算したい2つ目の数値を入力します。".to_string(),
                                "「計算する」ボタンをクリックすると計算結果が表示されます。".to_string(),
                                "計算結果は「計算結果」欄に表示されます。計算式と結果の両方が表示されます。".to_string(),
                            ],
                        },
                        UsageSectionProps {
                            title: "大きな数値の入力".to_string(),
                            items: vec![
                                "このツールは非常に大きな数値（通常の電卓では扱えないような桁数）を扱うことができます。".to_string(),
                                "数値は文字列として処理されるため、桁数の制限はほぼありません。".to_string(),
                                "小数点を含む数値も入力可能です。例: 「123456789.987654321」".to_string(),
                                "負の数を入力する場合は、数値の前に「-」を付けてください。例: 「-9876543210」".to_string(),
                            ],
                        },
                        UsageSectionProps {
                            title: "対応している演算".to_string(),
                            items: vec![
                                "加算（+）: 2つの数値を足し合わせます。".to_string(),
                                "減算（-）: 最初の数値から2つ目の数値を引きます。".to_string(),
                                "乗算（×）: 2つの数値を掛け合わせます。".to_string(),
                                "除算（÷）: 最初の数値を2つ目の数値で割ります。".to_string(),
                            ],
                        },
                        UsageSectionProps {
                            title: "注意事項".to_string(),
                            items: vec![
                                "0で割り算を行うと「0で除算できません」というエラーが表示されます。".to_string(),
                                "入力値が空の場合は、「最初の数値を入力してください」または「2つ目の数値を入力してください」というエラーが表示されます。".to_string(),
                                "Enterキーを押すことでも計算を実行できます。".to_string(),
                                "非常に大きな計算結果は、スクロールして全体を確認できます。".to_string(),
                            ],
                        },
                    ],
                }
            }
        }
    }
}

/// 式を計算して結果を更新
#[allow(dead_code)]
fn calculate(expr: &str, result: &mut Signal<String>, error: &mut Signal<String>) {
    if expr.is_empty() {
        result.set(String::from("0"));
        error.set(String::new());
        return;
    }

    match crate::libs::calculator::big_numbers::calculate_expression(expr) {
        Ok(value) => {
            result.set(value);
            error.set(String::new());
        }
        Err(err) => {
            error.set(err);
        }
    }
}
