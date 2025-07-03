# WebTools - Webアプリケーション

WebToolsのフロントエンドアプリケーションです。Rust + Dioxusフレームワークを使用して構築されており、Tailwind CSSでスタイリングされています。

## 🚀 クイックスタート

### 前提条件

- **Docker** と **Docker Compose** がインストールされていること
- プロジェクトルートから実行すること

### 開発環境の起動

```bash
# プロジェクトルートで開発環境を起動
docker compose up -d

# 開発サーバーの起動
docker compose exec web dx serve --features development --addr 0.0.0.0 --platform web
```

ブラウザで http://localhost:8080 を開いてアプリケーションにアクセスできます。

## 🛠️ 開発コマンド

### コード品質管理

```bash
# コードフォーマット（必須）
docker compose exec web cargo fmt

# リンター実行（全ての警告を修正すること）
docker compose exec web cargo clippy

# テスト実行
docker compose exec web cargo test
```

### ビルド

```bash
# 開発ビルド
docker compose exec web dx build --platform web

# プロダクションビルド（静的サイト生成）
docker compose exec web ./bundle.sh
```

プロダクションビルドは静的サイトジェネレーション（SSG）を使用してWebプラットフォーム向けの最適化されたビルドを作成します。

## 📁 プロジェクト構造

```
web/
├── src/                    # Rustソースコード
│   ├── components/         # 再利用可能なコンポーネント
│   ├── pages/              # ページコンポーネント
│   ├── utils/              # ユーティリティ関数
│   └── main.rs             # アプリケーションエントリーポイント
├── assets/                 # 静的アセット
│   ├── icons/              # アイコンファイル
│   ├── images/             # 画像ファイル
│   └── tailwind.css        # コンパイル済みTailwind CSS
├── docs/                   # ドキュメント
├── resources/              # リソースファイル
├── dist/                   # ビルド出力
├── input.css               # Tailwind CSSソース
├── bundle.sh               # プロダクションビルドスクリプト
├── Cargo.toml              # Rust依存関係
├── Dioxus.toml             # Dioxus設定
└── package.json            # Node.js依存関係（Tailwind CSS用）
```

## 🎨 スタイリング

このプロジェクトはTailwind CSS v4を使用しています。スタイルの変更は自動的に監視され、リアルタイムで反映されます。

### Tailwind CSS設定

- **入力ファイル**: `input.css`
- **出力ファイル**: `assets/tailwind.css`
- **自動コンパイル**: Docker Composeで自動実行

## 🧪 テスト

```bash
# 全テスト実行
docker compose exec web cargo test

# 特定のテスト実行
docker compose exec web cargo test test_name

# テストカバレッジ（cargo-llvm-covを使用）
docker compose exec web cargo install cargo-llvm-cov
docker compose exec web cargo llvm-cov --html
# カバレッジレポートは target/llvm-cov/html/index.html で確認可能
```

## 📝 開発ガイドライン

### コンポーネント作成

```rust
// Props定義
#[derive(Props)]
pub struct ComponentNameProps {
    title: String,
    #[props(default = false)]
    is_active: bool,
}

// コンポーネント実装
pub(crate) fn ComponentName(cx: Scope<ComponentNameProps>) -> Element {
    // 1. Props取得
    let title = &cx.props.title;
    let is_active = cx.props.is_active;

    // 2. State initialization
    let state = use_state(cx, || initial_value);

    // 3. Event handlers
    let handle_click = move |_| {
        // Event logic here
    };

    // 4. UI rendering
    cx.render(rsx! {
        div {
            class: "tailwind-classes {is_active.then(|| \"active\")}",
            h2 { "{title}" }
            button {
                onclick: handle_click,
                "Click me"
            }
        }
    })
}

// 使用例
rsx! {
    ComponentName {
        title: "サンプルタイトル",
        is_active: true
    }
}
```

### 命名規則

- **コンポーネント**: PascalCase (`UserProfile`)
- **関数・変数**: snake_case (`calculate_result`)
- **定数**: SCREAMING_SNAKE_CASE (`MAX_ITEMS`)

### 必須事項

- 全ての新機能には使用方法を説明する`Usage`コンポーネントを含める
- エラーハンドリングを適切に実装する
- コミット前に必ず`cargo fmt`と`cargo clippy`を実行する

## 🔗 関連ドキュメント

- **[開発ガイドライン](../.junie/guidelines.md)**: 包括的な開発・運用ガイドライン
- **[プロジェクト概要](../README.md)**: プロジェクト全体の概要
- **[CDKドキュメント](../cdk/docs/README.md)**: インフラストラクチャ関連ドキュメント
