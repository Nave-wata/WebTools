# Webアプリケーション開発ドキュメント

## 概要

このドキュメントは、Dioxusフレームワークを使用したWebアプリケーションの開発ガイドラインと規約をまとめたものです。プロジェクトの一貫性を保ち、開発効率を向上させるために参照してください。

## ドキュメント一覧

- [プロジェクト構造](./project_structure.md) - ディレクトリ構造と命名規則
- [コーディングガイドライン](./coding_guidelines.md) - コーディング規約とベストプラクティス
- [コンポーネントテンプレート](./component_template.md) - 新規コンポーネント作成用テンプレート
- [Docker開発環境](./docker_development.md) - Docker環境の詳細と使用方法

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
