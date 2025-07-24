use crate::routes::Route;
use dioxus::prelude::*;

/// ハンバーガーメニューのプロパティ
#[derive(PartialEq, Clone, Props)]
pub(crate) struct HamburgerMenuProps {
    /// メニューの表示状態
    is_open: Signal<bool>,
}

/// カテゴリデータ構造
#[derive(Clone, PartialEq)]
struct CategoryData {
    name: String,
    route: Route,
    tools: Vec<ToolData>,
}

/// ツールデータ構造
#[derive(Clone, PartialEq)]
struct ToolData {
    name: String,
    route: Route,
}

/// ハンバーガーメニューコンポーネント
///
/// 階層構造のナビゲーションメニューを提供します。
/// - カテゴリ一覧の表示
/// - カテゴリの展開/折りたたみ
/// - ツール一覧の表示
/// - リンク機能
pub(crate) fn HamburgerMenu(props: HamburgerMenuProps) -> Element {
    // 各カテゴリの展開状態を管理
    let mut generator_expanded = use_signal(|| false);
    let mut counter_expanded = use_signal(|| false);
    let mut converter_expanded = use_signal(|| false);
    let mut calculator_expanded = use_signal(|| false);
    let mut formatter_expanded = use_signal(|| false);

    // カテゴリデータを定義
    let categories = vec![
        CategoryData {
            name: "ジェネレータ".to_string(),
            route: Route::GeneratorPage {},
            tools: vec![
                ToolData {
                    name: "パスワード生成".to_string(),
                    route: Route::PasswordGenerator {},
                },
            ],
        },
        CategoryData {
            name: "カウンター".to_string(),
            route: Route::CounterPage {},
            tools: vec![
                ToolData {
                    name: "文字数カウント".to_string(),
                    route: Route::TextLengthCounter {},
                },
            ],
        },
        CategoryData {
            name: "コンバーター".to_string(),
            route: Route::ConverterPage {},
            tools: vec![
                ToolData {
                    name: "URL エンコード/デコード".to_string(),
                    route: Route::UrlEnDecoder {},
                },
                ToolData {
                    name: "Base エンコード/デコード".to_string(),
                    route: Route::BaseEnDecoder {},
                },
                ToolData {
                    name: "進数変換".to_string(),
                    route: Route::NumberBaseConverter {},
                },
            ],
        },
        CategoryData {
            name: "計算機".to_string(),
            route: Route::CalculatorPage {},
            tools: vec![
                ToolData {
                    name: "バイト単位変換".to_string(),
                    route: Route::ByteUnitCalculator {},
                },
                ToolData {
                    name: "シンプル電卓".to_string(),
                    route: Route::SimpleCalculator {},
                },
                ToolData {
                    name: "大きな数値の電卓".to_string(),
                    route: Route::BigNumbersCalculator {},
                },
            ],
        },
        CategoryData {
            name: "フォーマッター".to_string(),
            route: Route::FormatterPage {},
            tools: vec![
                ToolData {
                    name: "JSON フォーマッター".to_string(),
                    route: Route::JsonFormatter {},
                },
            ],
        },
    ];

    // 展開状態の配列を作成（カテゴリごとに対応）
    let expanded_states = vec![
        generator_expanded,
        counter_expanded,
        converter_expanded,
        calculator_expanded,
        formatter_expanded,
    ];

    rsx! {
        if props.is_open() {
            div {
                class: "fixed inset-0 z-50 bg-black bg-opacity-50",
                onclick: move |_| props.is_open.set(false),

                div {
                    class: "fixed top-0 left-0 h-full w-80 max-w-[80vw] bg-white shadow-lg transform transition-transform duration-300 ease-in-out",
                    onclick: move |e| e.stop_propagation(),

                    // ヘッダー
                    div {
                        class: "flex items-center justify-between p-4 border-b border-gray-200",
                        h2 {
                            class: "text-lg font-semibold text-gray-800",
                            "メニュー"
                        }
                        button {
                            class: "p-2 text-gray-500 hover:text-gray-700",
                            onclick: move |_| props.is_open.set(false),
                            "✕"
                        }
                    }

                    // メニュー内容
                    nav {
                        class: "p-4 overflow-y-auto h-full",

                        ul {
                            class: "space-y-2",

                            // トップページ
                            li {
                                Link {
                                    to: Route::TopPage {},
                                    class: "block px-3 py-2 text-gray-700 hover:bg-gray-100 rounded-md",
                                    onclick: move |_| props.is_open.set(false),
                                    "トップ"
                                }
                            }

                            // カテゴリ一覧
                            for (index, category) in categories.iter().enumerate() {
                                li {
                                    key: "{category.name}",

                                    // カテゴリヘッダー
                                    div {
                                        class: "flex items-center justify-between",

                                        Link {
                                            to: category.route.clone(),
                                            class: "flex-1 px-3 py-2 text-gray-700 hover:bg-gray-100 rounded-md",
                                            onclick: move |_| props.is_open.set(false),
                                            "{category.name}"
                                        }

                                        button {
                                            class: "p-2 text-gray-500 hover:text-gray-700",
                                            onclick: move |_| {
                                                let mut state = expanded_states[index];
                                                state.set(!state());
                                            },
                                            if expanded_states[index]() {
                                                "▼"
                                            } else {
                                                "▶"
                                            }
                                        }
                                    }

                                    // ツール一覧（展開時）
                                    if expanded_states[index]() {
                                        ul {
                                            class: "ml-6 mt-2 space-y-1",
                                            for tool in &category.tools {
                                                li {
                                                    key: "{tool.name}",
                                                    Link {
                                                        to: tool.route.clone(),
                                                        class: "block px-3 py-2 text-sm text-gray-600 hover:bg-gray-50 rounded-md",
                                                        onclick: move |_| props.is_open.set(false),
                                                        "{tool.name}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}