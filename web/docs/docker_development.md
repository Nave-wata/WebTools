# Docker開発環境

## 概要

このプロジェクトでは、Docker環境を使用して開発を行います。これにより、すべての開発者が同一の環境でコードを実行できるようになります。

## Docker環境の構成

### サービス構成

`docker-compose.yml` ファイルでは、以下のサービスが定義されています：

1. **web** - Rustアプリケーション実行環境
   - ベースイメージ: `ghcr.io/nave-wata/webtools/web:latest`
   - ポート: 8080（環境変数 `APP_PORT` で変更可能）
   - Dioxusアプリケーションの開発・ビルド環境

2. **tailwind** - TailwindCSS処理環境
   - ベースイメージ: `node:22.9.0`
   - CSSファイルの変更を監視し、自動的にビルド

## 使用方法

### 環境変数

`.env` ファイルまたは環境変数で以下の設定が可能です：

- `APP_PORT`: Webアプリケーションのポート（デフォルト: 8080）

### 基本的なコマンド

```bash
# 環境の起動
docker compose up -d

# 環境の停止
docker compose down

# 環境の再構築
docker compose up -d --build

# コンテナのログ確認
docker compose logs -f
```

### 開発作業

```bash
# Webコンテナ内でシェルを起動
docker compose exec web bash

# 特定のコマンドを実行
docker compose exec web cargo add some-package

# Tailwindコンテナのログを確認
docker compose logs -f tailwind
```

## Dockerコンテナのカスタマイズ

プロジェクトのDockerfile は `ghcr/web/Dockerfile` にあります。環境に新しいツールやライブラリを追加する必要がある場合は、このファイルを編集し、イメージを再ビルドしてください。

```bash
# カスタムイメージのビルド
cd ghcr/web
docker build -t ghcr.io/nave-wata/webtools/web:custom .

# カスタムイメージを使用するようdocker-compose.ymlを編集
# services.web.image を変更:
# image: "ghcr.io/nave-wata/webtools/web:custom"
```

## トラブルシューティング

### コンテナが起動しない場合

```bash
# ログを確認
docker compose logs

# 個別のサービスログを確認
docker compose logs web
docker compose logs tailwind
```

### ポートの競合がある場合

`.env` ファイルを作成または編集して、異なるポートを指定してください：

```
APP_PORT=8081
```

### ボリュームの問題

ファイル権限の問題が発生した場合は、以下のコマンドでボリュームをリセットしてください：

```bash
docker compose down -v
docker compose up -d
```

### キャッシュのクリア

ビルドに問題がある場合、キャッシュをクリアしてみてください：

```bash
# Rustのビルドキャッシュをクリア
docker compose exec web cargo clean

# Node.jsのキャッシュをクリア
docker compose exec tailwind npm cache clean --force
```
