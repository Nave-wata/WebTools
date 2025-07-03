# WebTools

WebToolsは、様々な便利なウェブツールを提供するプラットフォームです。Rustで書かれたDioxusフレームワークを使用して構築されており、AWS CDKを使用してAWSにデプロイされます。

## 🚀 主な機能

- **ランダムパスワード生成ツール**: 英字（大文字・小文字）、数字、記号を組み合わせたカスタマイズ可能なパスワードを生成
- **その他のユーティリティツール**: 開発者や一般ユーザー向けの便利なWebツール群

## 📁 プロジェクト構成

```
WebTools/
├── web/                    # Dioxusフロントエンドアプリケーション
│   ├── src/               # Rustソースコード
│   ├── assets/            # 静的アセット
│   ├── docs/              # Webアプリケーション関連ドキュメント
│   └── README.md          # Web開発ガイド
├── cdk/                   # AWS CDKインフラストラクチャ
│   ├── lib/               # CDK構成要素とスタック
│   ├── docs/              # CDK詳細ドキュメント
│   └── README.md          # CDK概要
├── .junie/                # 開発ガイドライン
│   └── guidelines.md      # 開発・運用ガイドライン
└── docker-compose.yml     # 開発環境設定
```

## 🛠️ 開発環境のセットアップ

### 前提条件

- **Docker** と **Docker Compose** がインストールされていること
- **Git** がインストールされていること

### 開発サーバーの起動

```bash
# 開発環境の起動
docker compose up -d

# 開発サーバーの起動（Webコンテナ内で実行）
docker compose exec web dx serve --features development --addr 0.0.0.0 --platform web
```

これにより、以下のサービスが起動します：
- **Webアプリケーション**: http://localhost:8080 でアクセス可能
- **Tailwind CSSコンパイラ**: CSSの変更を自動監視・コンパイル

🔒 **セキュリティ注意**: 開発サーバーは開発環境でのみ使用してください。本番環境では使用しないでください。

### 開発コマンド

```bash
# コードフォーマット
docker compose exec web cargo fmt

# リンター実行
docker compose exec web cargo clippy

# テスト実行
docker compose exec web cargo test

# プロダクションビルド
docker compose exec web ./bundle.sh
```

## 🚀 デプロイ

プロジェクトのデプロイには、AWS CDKを使用します。詳細は [CDKドキュメント](./cdk/docs/README.md) を参照してください。

### クイックデプロイ

```bash
cd cdk
npm install
npm run build
npx cdk deploy --all
```

⚠️ **注意**: このコマンドはAWSリソースをプロビジョニングし、料金が発生する可能性があります。デプロイ前に必ず料金体系を確認してください。

## 📚 ドキュメント

### 開発関連
- **[開発ガイドライン](./.junie/guidelines.md)**: 開発・運用の包括的なガイドライン
- **[Webアプリケーション](./web/README.md)**: フロントエンド開発ガイド
- **[CDKインフラストラクチャ](./cdk/docs/README.md)**: インフラ構築・運用ガイド

### アーキテクチャ
- **[CDKアーキテクチャ](./cdk/docs/architecture.md)**: システム全体のアーキテクチャ
- **[構成要素](./cdk/docs/constructs.md)**: CDK構成要素の詳細仕様
- **[デプロイメント](./cdk/docs/deployment-guide.md)**: デプロイ手順とトラブルシューティング

## 🤝 開発ワークフロー

1. **機能ブランチの作成**: `git checkout -b feature/[feature-name]`
2. **開発**: コード作成・テスト・フォーマット
3. **品質チェック**: Clippy・テスト・ビルド確認
4. **コミット**: 日本語でのコミットメッセージ
5. **プルリクエスト**: GitHub CLIでPR作成

詳細は [開発ガイドライン](./.junie/guidelines.md) を参照してください。

## 🔧 技術スタック

- **フロントエンド**: Rust + Dioxus
- **スタイリング**: Tailwind CSS
- **インフラ**: AWS CDK (TypeScript)
- **開発環境**: Docker + Docker Compose
- **CI/CD**: GitHub Actions
