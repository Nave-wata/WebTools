# Webアプリケーション開発ドキュメント

## 概要

このドキュメントは、Dioxusフレームワークを使用したWebアプリケーションの開発ガイドラインと規約をまとめたものです。プロジェクトの一貫性を保ち、開発効率を向上させるために参照してください。

## ドキュメント一覧

- [プロジェクト構造](./project_structure.md) - ディレクトリ構造と命名規則
- [コーディングガイドライン](./coding_guidelines.md) - コーディング規約とベストプラクティス
- [コンポーネントテンプレート](./component_template.md) - 新規コンポーネント作成用テンプレート
- [Docker開発環境](./docker_development.md) - Docker環境の詳細と使用方法

## Rustのベストプラクティス

このプロジェクトでは、以下のRustのベストプラクティスを採用しています：

1. **コレクションの空チェック**: `collection.len() == 0` ではなく `collection.is_empty()` を使用
2. **範囲チェック**: `value < min || value > max` ではなく `!(min..=max).contains(&value)` を使用
3. **定数の型定義**: 文字列定数では `&'static str` ではなく `&str` を使用
4. **モジュールドキュメント**: モジュールには `//!` を、関数/構造体には `///` を使用
5. **シグナルの初期化**: 単純な値の場合は `use_signal(|| String::new())` ではなく `use_signal(String::new)` を使用
6. **変数の可変性**: 後で値を変更する変数には `mut` キーワードを使用

## よくある問題と解決策

### ビルドエラー

- **変数の可変性に関する問題**: 変数を変更する場合は、宣言時に `mut` キーワードを追加してください
- **ライフタイム指定の問題**: 定数の文字列リテラルでは、明示的な `'static` ライフタイムは不要です

### Docker環境の問題

- **Clippy修正の適用**: バージョン管理なしで修正を適用するには `--allow-no-vcs` フラグを使用してください
  ```bash
  docker compose exec web cargo clippy --fix --bin "WebTools" --allow-no-vcs
  ```

## 開発環境のセットアップ

### 必要なツール

- Docker
- Docker Compose

### 開発環境の起動

```bash
# 開発環境の起動
docker compose up -d

# コンテナの状態確認
docker compose ps
```

これにより、以下のサービスが起動します：
- Webアプリケーション - デフォルトでは8080ポートでアクセス可能 (`http://localhost:8080`)
- Tailwind CSSコンパイラ - CSSの変更を監視

### 開発サーバーの操作

```bash
# 開発サーバーの起動
docker compose exec web dx serve --features development --addr 0.0.0.0 --platform web

# コンテナ内でコマンドを実行する（例: cargo fmt）
docker compose exec web cargo fmt

# コンテナ内のシェルを起動
docker compose exec web bash
```

### ビルド

```bash
docker compose exec web ./bundle.sh
```
