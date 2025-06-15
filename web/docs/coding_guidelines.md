# コーディングガイドライン

## 一般原則

1. **明確さを優先**: 短いコードよりも読みやすいコードを優先します
2. **コメント**: 複雑なロジックには適切なコメントを付けます
3. **単一責任**: 各関数・コンポーネントは単一の責任を持つようにします
4. **DRY (Don't Repeat Yourself)**: コードの重複を避けます

## Rustコーディング規約

### フォーマット

- `cargo fmt` を使用して一貫したフォーマットを維持します（`docker compose exec web cargo fmt`）
- インデントは4スペースを使用します

### 命名規則

- **型名** (構造体、列挙型、トレイト): パスカルケース (`SimpleCalculator`)
- **変数と関数**: スネークケース (`calculate_result`)
- **定数**: 大文字のスネークケース (`MAX_LENGTH`)
- **ライフタイムパラメータ**: 短い小文字 (`'a`, `'b`)

### エラー処理

- パニックではなく `Result` を返す関数を優先します
- エラーメッセージは具体的で有用な情報を提供します

### 可視性

- 可能な限り最小限の可視性を使用します (`pub` vs `pub(crate)` vs 非公開)
- 必要なものだけをエクスポートします

## Dioxus コンポーネント規約

### コンポーネント構造

```rust
pub(crate) fn ComponentName(cx: Scope) -> Element {
    // 状態の初期化
    let state = use_state(cx, || initial_value);

    // イベントハンドラ
    let handle_event = move |_| {
        // イベント処理
    };

    // UI レンダリング
    cx.render(rsx! {
        div {
            class: "style-classes",
            // コンポーネントの内容
        }
    })
}
```

### 状態管理

- 複雑な状態は適切に分割します
- 状態更新ロジックは明確に分離します
- 共有状態には `use_shared_state` を使用します

### イベント処理

- イベントハンドラはコンポーネント関数内で定義し、`move` クロージャを使用します
- 複雑なイベント処理は別の関数に分離します

### 条件付きレンダリング

```rust
rsx! {
    // 方法1: if 式を使用
    { if condition { rsx!(div { "Conditional content" }) } else { None } }

    // 方法2: マッチング
    { match state.get() {
        State::A => rsx!(div { "State A" }),
        State::B => rsx!(div { "State B" }),
    }}
}
```

### スタイリング

- Tailwind CSSクラスを使用します
- 共通のスタイルは抽出して再利用します
- レスポンシブデザインを考慮してスタイルを適用します

### 使い方コンポーネント

全てのツールページには、ユーザーが機能を理解し効果的に使用できるように使い方説明を追加する必要があります。
`Usage` コンポーネントを使用して、一貫したスタイルと構造で使い方を表示してください：

```rust
// インポート
use crate::components::instructions::usage::{Usage, UsageSectionProps};

// コンポーネント内で使用
Usage {
    sections: vec![
        UsageSectionProps {
            title: "基本的な使い方".to_string(),
            items: vec![
                "このツールの基本的な使い方の説明を記述します。".to_string(),
                "箇条書きで操作手順を説明します。".to_string(),
            ],
        },
        // 必要に応じて追加のセクションを追加
    ],
}
```

`Usage` コンポーネントは以下の特徴を持っています：
- 一貫したスタイルと構造で使い方を表示
- セクションごとに分けて説明を整理
- 箇条書きで操作手順を明確に表示
- レスポンシブデザインに対応

## テスト規約

### ユニットテスト

- 各モジュールには対応するテストを作成します
- テスト関数名は `test_機能名_条件_期待される結果` の形式にします
- モックとスタブを適切に使用します

### コンポーネントテスト

- 各コンポーネントの主要な機能をテストします
- レンダリング結果と状態変化の両方をテストします

## ドキュメント

- 公開APIには常にドキュメントコメント (`///`) を付けます
- 例や使用方法を含めます
- モジュールレベルのドキュメント (`//!`) も活用します

## 性能最適化

- 不必要な再レンダリングを避けます
- メモ化を適切に使用します (`use_memo`)
- 大きなリストはページネーションや仮想スクロールを検討します

## Rustのベストプラクティス

### コレクションの空チェック

コレクション（文字列、ベクター等）が空かどうかを確認する場合は、長さを0と比較するのではなく、`is_empty()` メソッドを使用します：

```rust
// 非推奨
if vec_chars.len() == 0 {
    // 処理
}

// 推奨
if vec_chars.is_empty() {
    // 処理
}
```

### 範囲チェック

変数が特定の範囲内にあるかどうかを確認する場合は、複数の比較演算子を使用するのではなく、Rustの範囲構文と `contains` メソッドを使用します：

```rust
// 非推奨
if base < 2 || base > 36 {
    // エラー処理
}

// 推奨
if !(2..=36).contains(&base) {
    // エラー処理
}
```

### 定数の型定義

Rust 2018以降では、文字列リテラルの型は自動的に `&'static str` と推論されるため、明示的に指定する必要はありません：

```rust
// 非推奨
pub(crate) const APP_TITLE: &'static str = "アプリタイトル";

// 推奨
pub(crate) const APP_TITLE: &str = "アプリタイトル";
```

### モジュールドキュメント

モジュールレベルのドキュメントには `//!` を使用し、関数/構造体/メソッドのドキュメントには `///` を使用します：

```rust
//! このモジュールは数値の進数変換機能を提供します。
//! 
//! 様々な基数間での変換をサポートしています。

/// 10進数の文字列を指定された進数に変換する
///
/// # Arguments
///
/// * `decimal_str` - 10進数の文字列
/// * `base` - 変換先の進数 (2-36)
pub fn decimal_to_base(decimal_str: &str, base: u32) -> Result<String, String> {
    // 実装
}
```

### シグナルの初期化

Dioxusのシグナルを初期化する際、単純な値の場合は冗長なクロージャを避け、より簡潔な初期化構文を使用します：

```rust
// 冗長
let mut error_message = use_signal(|| String::new());

// 簡潔
let mut error_message = use_signal(String::new);
```

## トラブルシューティング

### ビルドエラー

#### 変数の可変性に関する問題

**症状**:
```
error[E0384]: cannot assign twice to immutable variable `builder`
```

**原因**: 変数が不変（immutable）として宣言されているが、後でその値を変更しようとしている。

**解決策**: 変数宣言時に `mut` キーワードを追加して可変にする：

```rust
// 誤り
let builder = LaunchBuilder::new();
builder = builder.with_cfg(...); // エラー: 不変変数に再代入できない

// 正しい
let mut builder = LaunchBuilder::new();
builder = builder.with_cfg(...); // OK: 可変変数なので再代入可能
```

### Docker環境の問題

#### Clippy修正の適用に関する問題

**症状**:
```
error: cannot apply fixes without version control
```

**原因**: Clippyの自動修正機能はデフォルトでバージョン管理システム（Git）を使用している環境でのみ動作する。

**解決策**: `--allow-no-vcs` フラグを追加して、バージョン管理なしでも修正を適用できるようにする：

```bash
docker compose exec web cargo clippy --fix --bin "WebTools" --allow-no-vcs
```
