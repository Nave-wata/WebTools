# 新規ルート追加ガイド

このドキュメントでは、WebToolsプロジェクトに新しいツールを追加するための完全なプロセスを説明します。

## 目次

1. [概要](#概要)
2. [ステップ1: ルートファイルの作成](#ステップ1-ルートファイルの作成)
3. [ステップ2: ルーティング設定の追加](#ステップ2-ルーティング設定の追加)
4. [ステップ3: カードコンポーネントの作成](#ステップ3-カードコンポーネントの作成)
5. [ステップ4: トップページへのカード追加](#ステップ4-トップページへのカード追加)
6. [ステップ5: アイコンとOGP画像の追加](#ステップ5-アイコンとogp画像の追加)
7. [ステップ6: 必要に応じたグリッドの追加](#ステップ6-必要に応じたグリッドの追加)
8. [完全な例](#完全な例)

## 概要

新しいツールを追加するには、以下の主要なステップが必要です：

1. ルートファイルの作成（ツールの実装）
2. ルーティング設定の追加
3. カードコンポーネントの作成
4. トップページへのカード追加
5. アイコンとOGP画像の追加
6. 必要に応じたグリッドの追加

以下、各ステップを詳細に説明します。

## ステップ1: ルートファイルの作成

まず、新しいツールのルートファイルを作成します。ファイルは適切なカテゴリのディレクトリに配置します。

例えば、新しい「テキスト変換」ツールを追加する場合：

```
/web/src/routes/converter/text_transform.rs
```

ファイルの基本構造は以下のようになります：

```rust
use crate::components::breadcrumb::{BreadcrumbItem, BreadcrumbList};
use crate::components::head::Head;
use crate::routes::Route;
use dioxus::prelude::*;

/// テキスト変換ツールコンポーネント
pub(crate) fn TextTransformer() -> Element {
    // メタ変数
    let title: &str = "テキスト変換";
    let description: &str = "テキストを様々な形式に変換するツールです。";

    // 状態の初期化
    let input = use_state(String::new);
    let output = use_state(String::new);

    // イベントハンドラ
    let transform_text = move |_| {
        // 変換ロジックを実装
        let transformed = input().to_uppercase(); // 例: 大文字に変換
        output.set(transformed);
    };

    // UI レンダリング
    rsx! {
        Head {
            title: title,
            description: description,
            og_url: Route::TextTransformer {}.to_string(),
            og_image: asset!("/assets/images/ogp/text_transformer.webp"),
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

            // ツールの実装
            // ...
        }
    }
}
```

## ステップ2: ルーティング設定の追加

次に、`routes.rs`ファイルに新しいルートを追加します：

1. モジュールの宣言を追加：

```rust
mod converter;  // 既存の場合は追加不要
```

2. 新しいコンポーネントのインポートを追加：

```rust
use converter::text_transform::TextTransformer;
```

3. `Route`列挙型に新しいルートを追加：

```rust
#[derive(Clone, Routable, Debug, PartialEq)]
pub(crate) enum Route {
    #[layout(DefaultLayout)]
        // 既存のルート...

        #[nest("/converter")]
            // 既存のコンバーターツール...

            #[route("/text-transform")]
            TextTransformer {},
        #[end_nest]
    #[end_layout]

    // その他のルート...
}
```

## ステップ3: カードコンポーネントの作成

トップページに表示するためのカードコンポーネントを作成します：

```
/web/src/components/cards/text_transformer_card.rs
```

```rust
use crate::components::cards::tool_card::ToolCard;
use crate::routes::Route;
use dioxus::prelude::*;

/// テキスト変換ツールカード
pub(crate) fn TextTransformerCard() -> Element {
    let icon: Element = rsx! {
        img {
            src: asset!("/assets/images/icons/text_transformer.webp"),
            alt: "テキスト変換",
            class: "justify-self-center py-3",
            height: 60,
            width: 150,
        }
    };

    rsx! {
        ToolCard {
            to: Route::TextTransformer {},
            icon: icon,
            title: "テキスト変換",
            description: vec![
                vec![
                    "テキストを様々な形式に".to_string(),
                    "変換することができます。".to_string(),
                ],
                vec![
                    "大文字・小文字変換や".to_string(),
                    "特殊文字の置換などに対応。".to_string(),
                ],
            ],
        }
    }
}
```

また、`components/cards.rs`ファイルにモジュールを追加します：

```rust
pub(crate) mod text_transformer_card;
```

## ステップ4: トップページへのカード追加

`routes/top.rs`ファイルを編集して、トップページに新しいカードを追加します：

1. カードコンポーネントのインポートを追加：

```rust
use crate::components::cards::text_transformer_card::TextTransformerCard;
```

2. 適切なToolsGridにカードを追加：

```rust
// routes/top.rs 内の rsx! マクロ内で
rsx! {
    // 他のコード...

    ToolsGrid {
        title: "コンバーター",

        NumberBaseConverterCard {},
        TextTransformerCard {},  // 新しいカードを追加
    }

    // 他のコード...
}
```

## ステップ5: アイコンとOGP画像の追加

新しいツールのアイコンとOGP画像を追加します：

1. アイコン画像を追加：
   `/web/assets/images/icons/text_transformer.webp`

2. OGP画像を追加：
   `/web/assets/images/ogp/text_transformer.webp`

画像は適切なサイズとフォーマットで作成してください。

## ステップ6: 必要に応じたグリッドの追加

ツールの機能によっては、新しいグリッドコンポーネントが必要になる場合があります。その場合は、`components/grids`ディレクトリに新しいグリッドコンポーネントを作成します。

例えば、テキスト変換結果を表示するためのグリッドが必要な場合：

```
/web/src/components/grids/text_transform_grid.rs
```

```rust
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub(crate) struct TextTransformGridProps {
    original: String,
    transformed: String,
}

pub(crate) fn TextTransformGrid(props: TextTransformGridProps) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-2 gap-4",

            div {
                class: "border p-3 rounded",
                h3 { class: "font-bold", "元のテキスト" }
                pre { class: "mt-2 whitespace-pre-wrap", {props.original.clone()} }
            }

            div {
                class: "border p-3 rounded",
                h3 { class: "font-bold", "変換後のテキスト" }
                pre { class: "mt-2 whitespace-pre-wrap", {props.transformed.clone()} }
            }
        }
    }
}
```

また、`components/grids.rs`ファイルにモジュールを追加します：

```rust
pub(crate) mod text_transform_grid;
```

## 完全な例

完全な例として、パスワード生成ツールの実装を参照してください：

- ルートファイル: `/web/src/routes/generator/password.rs`
- カードコンポーネント: `/web/src/components/cards/password_generator_card.rs`
- トップページでの使用: `/web/src/routes/top.rs`

これらのファイルは、新しいツールを追加する際の参考になります。
