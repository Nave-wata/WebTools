# トラブルシューティングガイド

## 概要

このドキュメントでは、開発中によく遭遇する問題とその解決方法を説明します。ビルドエラー、実行時エラー、およびその他の一般的な問題に対処するためのガイダンスを提供します。

## ビルドエラー

### 変数の可変性に関する問題

#### 症状
```
error[E0384]: cannot assign twice to immutable variable `builder`
```

#### 原因
変数が不変（immutable）として宣言されているが、後でその値を変更しようとしている。

#### 解決策
変数宣言時に `mut` キーワードを追加して可変にする：

```rust
// 誤り
let builder = LaunchBuilder::new();
builder = builder.with_cfg(...); // エラー: 不変変数に再代入できない

// 正しい
let mut builder = LaunchBuilder::new();
builder = builder.with_cfg(...); // OK: 可変変数なので再代入可能
```

### ライフタイム指定の問題

#### 症状
```
error: unnecessary lifetime parameter
```

#### 原因
定数宣言で不要な静的ライフタイム（`'static`）が指定されている。

#### 解決策
Rust 2018以降では、文字列リテラルの型は自動的に `&'static str` と推論されるため、明示的に指定する必要はありません：

```rust
// 誤り
pub(crate) const APP_TITLE: &'static str = "アプリタイトル";

// 正しい
pub(crate) const APP_TITLE: &str = "アプリタイトル";
```

## コード品質の問題

### 長さチェックの非効率なパターン

#### 症状
```
warning: length comparison to zero
```

#### 原因
コレクション（文字列、ベクター等）の長さを0と比較している。

#### 解決策
より明確で効率的な `is_empty()` メソッドを使用する：

```rust
// 非推奨
if vec_chars.len() == 0 {
    // ...
}

// 推奨
if vec_chars.is_empty() {
    // ...
}
```

### 範囲チェックの冗長な記述

#### 症状
複数の比較演算子を使用した範囲チェック。

#### 原因
変数が特定の範囲内にあるかどうかを確認するために、複数の比較演算子を使用している。

#### 解決策
Rustの範囲構文と `contains` メソッドを使用して、より簡潔で読みやすいコードにする：

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

## ドキュメンテーションの問題

### モジュールドキュメントの形式

#### 症状
モジュールレベルのドキュメントが関数ドキュメントと同じ形式（`///`）で書かれている。

#### 原因
モジュールドキュメントと関数/構造体ドキュメントの違いを理解していない。

#### 解決策
モジュールレベルのドキュメントには `//!` を使用し、関数/構造体/メソッドのドキュメントには `///` を使用する：

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

## Dioxus固有の問題

### シグナルの初期化パターン

#### 症状
冗長なクロージャを使用したシグナル初期化。

#### 原因
古いパターンでシグナルを初期化している。

#### 解決策
単純な値の場合、より簡潔な初期化構文を使用する：

```rust
// 冗長
let mut error_message = use_signal(|| String::new());

// 簡潔
let mut error_message = use_signal(String::new);
```

## Docker環境の問題

### ビルドスクリプトの実行権限

#### 症状
```
bash: ./bundle.sh: Permission denied
```

#### 原因
ビルドスクリプトに実行権限がない。

#### 解決策
スクリプトに実行権限を付与する：

```bash
docker compose exec web chmod +x ./bundle.sh
```

### Clippy修正の適用に関する問題

#### 症状
```
error: cannot apply fixes without version control
```

#### 原因
Clippyの自動修正機能はデフォルトでバージョン管理システム（Git）を使用している環境でのみ動作する。

#### 解決策
`--allow-no-vcs` フラグを追加して、バージョン管理なしでも修正を適用できるようにする：

```bash
docker compose exec web cargo clippy --fix --bin "WebTools" --allow-no-vcs
```
