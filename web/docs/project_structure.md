# プロジェクト構造ドキュメント

## ディレクトリ構造

```
/web
├── src/                  # ソースコードディレクトリ
│   ├── components/       # 再利用可能なUIコンポーネント
│   ├── constants/        # 定数定義
│   ├── libs/             # ユーティリティライブラリ
│   ├── routes/           # ルーティング定義とページコンポーネント
│   ├── components.rs     # コンポーネントのエクスポート
│   ├── constants.rs      # 定数のエクスポート
│   ├── libs.rs           # ライブラリのエクスポート
│   ├── main.rs           # アプリケーションのエントリーポイント
│   └── routes.rs         # ルーティング設定
├── assets/               # 静的アセット（画像、フォントなど）
├── resources/            # リソースファイル
└── docs/                 # ドキュメント（このディレクトリ）
```

## 命名規則

### ファイル名

- **モジュールファイル**: スネークケース（例: `text_length.rs`、`byte_unit.rs`）
- **エクスポートファイル**: 単数形のスネークケース（例: `components.rs`、`routes.rs`）

### コンポーネント

- **コンポーネント関数**: パスカルケース（例: `SimpleCalculator`、`PasswordGenerator`）
- **内部関数/メソッド**: スネークケース（例: `handle_input`、`calculate_result`）

### 変数

- **変数名**: スネークケース（例: `first_operand`、`display_value`）
- **定数**: 大文字のスネークケース（例: `MAX_LENGTH`、`DEFAULT_VALUE`）

## モジュール構成

### routes

アプリケーションのページコンポーネントを含むモジュール。各機能ごとにサブディレクトリに分割されています。

- `top`: トップページ
- `generator`: 生成ツール（パスワード生成器など）
- `counter`: カウンターツール（テキスト長さカウンターなど）
- `converter`: 変換ツール（数値基数変換など）
- `calculator`: 計算ツール（バイト単位計算機、シンプル計算機など）
- `notfound`: 404ページ

### components

再利用可能なUIコンポーネントを含むモジュール。

- `layouts`: レイアウトコンポーネント（例: `DefaultLayout`）
- `errors`: エラー関連のコンポーネント（例: `Err404`）
- `instructions`: 使い方説明関連のコンポーネント
  - `usage`: 使い方コンポーネント（`Usage`、`UsageSection`）

## ルーティング

Dioxusのルーティングシステムを使用しています。ルート定義は `routes.rs` の `Route` enumで定義され、各ルートは対応するコンポーネント関数にマッピングされています。

```rust
#[derive(Clone, Routable, Debug, PartialEq)]
pub(crate) enum Route {
    #[layout(DefaultLayout)]
        #[route("/")]
        TopPage {},

        #[nest("/generator")]
            #[route("/password")]
            PasswordGenerator {},
        #[end_nest]

        // その他のルート定義...
}
```

## コンポーネント実装パターン

各機能コンポーネントは、以下のパターンに従って実装されることが推奨されます：

1. 状態管理のための enum 定義（必要な場合）
2. コンポーネント関数の実装（パスカルケースで命名）
3. イベントハンドラの実装
4. UI レンダリングのコード

```rust
// 例: シンプル計算機コンポーネント
#[derive(Clone, PartialEq)]
enum CalculatorState {
    InputFirstOperand,
    InputOperator,
    InputSecondOperand,
    ShowResult,
}

pub(crate) fn SimpleCalculator() -> Element {
    // 状態管理とイベントハンドラの実装
    // ...

    // UI レンダリング
    rsx! {
        div { /* コンポーネントの UI */ }
    }
}
```

## スタイリング

Tailwind CSSを使用してスタイリングを行います。スタイル定義は各コンポーネント内の `class` 属性で直接指定します。

## ビルドプロセス

### 開発環境

```bash
# 開発サーバーの起動
docker compose up -d

# 開発サーバーを再起動する場合
docker compose exec web dx serve --features development --addr 0.0.0.0 --platform web
```

### 本番ビルド

```bash
# 本番用にビルド
docker compose exec web ./bundle.sh
```

このスクリプトは以下の処理を行います：
- TailwindCSSの最適化（未使用クラスの削除）
- Rustコードの本番向けビルド
- 静的ファイルの最適化

### コンテナでの開発作業

```bash
# Rustのコードフォーマット
docker compose exec web cargo fmt

# リンターの実行
docker compose exec web cargo clippy

# テストの実行
docker compose exec web cargo test
```
