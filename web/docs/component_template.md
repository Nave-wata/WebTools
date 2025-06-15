# コンポーネントテンプレート

## 基本コンポーネント

以下は新しいページコンポーネントを作成する際の基本テンプレートです。

```rust
use dioxus::prelude::*;
use crate::components::ui::heading::Heading;

/// コンポーネントの説明をここに記述
pub(crate) fn ComponentName() -> Element {
    // 状態の初期化
    let state = use_state(|| initial_value);

    // イベントハンドラ
    let handle_event = move |_| {
        // イベント処理
    };

    // UI レンダリング
    rsx! {
        div {
            class: "container mx-auto p-4",
            Heading {
                level: "h1",
                text: "ページタイトル"
            }

            // ページコンテンツ
            div {
                class: "mt-4",
                // コンポーネント内容
            }
        }
    }
}
```

## 計算機ツール系コンポーネント

計算系ツールコンポーネントのテンプレートです。

```rust
use dioxus::prelude::*;
use crate::components::ui::heading::Heading;
use crate::components::ui::card::Card;
use crate::components::instructions::usage::{Usage, UsageSectionProps};

#[derive(Clone, PartialEq)]
enum CalculatorState {
    Input,
    Processing,
    Result,
    Error,
}

pub(crate) fn CalculatorComponent() -> Element {
    // 状態の初期化
    let state = use_state(|| CalculatorState::Input);
    let input = use_state(String::new);
    let result = use_state(String::new);
    let error = use_state(String::new);

    // 計算処理
    let calculate = move |_| {
        state.set(CalculatorState::Processing);

        // 計算ロジック
        match perform_calculation(input.get()) {
            Ok(value) => {
                result.set(value.to_string());
                state.set(CalculatorState::Result);
            }
            Err(err) => {
                error.set(err.to_string());
                state.set(CalculatorState::Error);
            }
        }
    };

    // 入力ハンドラ
    let handle_input = move |evt: Event<FormData>| {
        input.set(evt.value.clone());
    };

    // リセットハンドラ
    let reset = move |_| {
        input.set(String::new());
        result.set(String::new());
        error.set(String::new());
        state.set(CalculatorState::Input);
    };

    // UI レンダリング
    rsx! {
        div {
            class: "container mx-auto p-4",
            Heading {
                level: "h1",
                text: "計算ツール名"
            }

            Card {
                class: "mt-4",

                // 説明文
                div {
                    class: "mb-4",
                    p { "このツールの説明文をここに記述します。" }
                }

                // 入力フォーム
                div {
                    class: "mb-4",
                    label {
                        class: "block text-sm font-medium text-gray-700 mb-1",
                        "入力:"
                    }
                    input {
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                        value: "{input}",
                        oninput: handle_input
                    }
                }

                // ボタン
                div {
                    class: "flex space-x-2 mb-4",
                    button {
                        class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                        onclick: calculate,
                        "計算"
                    }
                    button {
                        class: "px-4 py-2 bg-gray-300 text-gray-700 rounded hover:bg-gray-400",
                        onclick: reset,
                        "リセット"
                    }
                }

                // 結果表示
                {
                    match state.get() {
                        CalculatorState::Result => rsx! {
                            div {
                                class: "p-3 bg-green-50 border border-green-200 rounded",
                                p {
                                    class: "font-medium",
                                    "結果: {result}"
                                }
                            }
                        },
                        CalculatorState::Error => rsx! {
                            div {
                                class: "p-3 bg-red-50 border border-red-200 rounded",
                                p {
                                    class: "font-medium text-red-700",
                                    "エラー: {error}"
                                }
                            }
                        },
                        _ => rsx! { None }
                    }
                }

                // 使い方コンポーネント
                // 全てのツールページには使い方説明を追加してください
                Usage {
                    sections: vec![
                        UsageSectionProps {
                            title: "基本的な使い方".to_string(),
                            items: vec![
                                "このツールの基本的な使い方の説明を記述します。".to_string(),
                                "箇条書きで操作手順を説明します。".to_string(),
                            ],
                        },
                        UsageSectionProps {
                            title: "高度な機能".to_string(),
                            items: vec![
                                "高度な機能や特殊な使い方について説明します。".to_string(),
                                "ユーザーが知っておくべき注意点などを記述します。".to_string(),
                            ],
                        },
                    ],
                }
            }
        }
    }
}

// 計算ロジック（別関数に分離）
fn perform_calculation(input: &str) -> Result<f64, &'static str> {
    // 実際の計算ロジックを実装
    Ok(0.0) // 仮の戻り値
}
```

## 新規ルートの追加方法

新しいページコンポーネントを追加する手順:

1. 適切なサブディレクトリに新しいファイルを作成 (例: `routes/tools/new_tool.rs`)
   ```bash
   # Docker環境内で実行
   docker compose exec web bash -c "touch src/routes/tools/new_tool.rs"
   ```

2. モジュールの親ファイルでモジュールを宣言 (例: `routes/tools.rs` に `mod new_tool;` を追加)
3. 必要に応じて公開エクスポートを追加 (例: `pub use new_tool::NewTool;`)
4. `routes.rs` の `Route` enum に新しいルート定義を追加:

```rust
#[derive(Clone, Routable, Debug, PartialEq)]
pub(crate) enum Route {
    // 既存のルート...

    #[nest("/tools")]
        #[route("/new-tool")]
        NewTool {},
    #[end_nest]
}
```

5. 必要に応じてナビゲーションメニューに新しいリンクを追加
6. 開発サーバーまたはビルドを行い変更を確認
