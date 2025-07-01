# WebTools - AWS CDK インフラストラクチャ

WebToolsプロジェクトのAWS CDK (Cloud Development Kit) インフラストラクチャコードです。TypeScriptで記述されており、AWS上でのWebアプリケーションのホスティングとデプロイを自動化します。

## 🏗️ アーキテクチャ概要

このCDKプロジェクトは以下の3つのメインスタックで構成されています：

- **Domain Stack**: SSL証明書とDNS設定
- **Storage Stack**: S3バケットによる静的サイトホスティング
- **CDN Stack**: CloudFrontによるコンテンツ配信とセキュリティヘッダー

## 🚀 クイックスタート

### 前提条件

- **Node.js** (v18以上) がインストールされていること
- **AWS CLI** が設定されていること
- **AWS CDK** がインストールされていること (`npm install -g aws-cdk`)

### 環境設定

```bash
# CDKディレクトリに移動
cd cdk

# 依存関係のインストール
npm install

# 環境変数の設定
cp .env.example .env
# .envファイルを編集して必要な値を設定
```

#### 必要な環境変数

`.env`ファイルに以下の環境変数を設定してください：

```bash
# ドメイン設定
DOMAIN_NAME=your-domain.com
SUBDOMAIN=tools

# AWS設定
AWS_ACCOUNT_ID=123456789012
AWS_REGION=us-east-1

# 環境設定
ENVIRONMENT=development
```

### デプロイ

```bash
# TypeScriptのコンパイル
npm run build

# テストの実行
npm test

# CDK構文チェック
npx cdk synth

# 差分確認
npx cdk diff

# 全スタックのデプロイ
npx cdk deploy --all
```

## 🛠️ 開発コマンド

### ビルドとテスト

```bash
# TypeScriptコンパイル
npm run build

# ファイル変更監視モード
npm run watch

# Jestユニットテスト実行
npm test

# テストカバレッジ
npm run test:coverage
```

### CDK操作

```bash
# CloudFormationテンプレート生成
npx cdk synth

# デプロイ前の差分確認
npx cdk diff

# 特定のスタックのデプロイ
npx cdk deploy WebToolsDomainStack

# 全スタックのデプロイ
npx cdk deploy --all

# スタックの削除
npx cdk destroy --all
```

## 📁 プロジェクト構造

```
cdk/
├── bin/                    # CDKアプリケーションエントリーポイント
│   └── app.ts             # メインアプリケーション定義
├── lib/                   # CDK構成要素とスタック
│   ├── constructs/        # 再利用可能なCDK構成要素
│   │   ├── cdn/          # CloudFront関連
│   │   ├── domain/       # ドメイン・証明書関連
│   │   ├── shared/       # 共通構成要素
│   │   └── storage/      # ストレージ関連
│   ├── interfaces/        # TypeScript型定義
│   └── stacks/           # CDKスタック定義
├── test/                  # テストファイル
├── utils/                 # ユーティリティ関数
├── docs/                  # 詳細ドキュメント
├── assets/                # デプロイ用アセット
├── .env                   # 環境変数設定
└── .env.example           # 環境変数テンプレート
```

## 📚 詳細ドキュメント

包括的なドキュメントは `docs/` ディレクトリにあります：

### 🎯 主要ドキュメント
- **[📖 CDKドキュメント概要](./docs/README.md)**: 全ドキュメントの入り口
- **[🏗️ アーキテクチャ](./docs/architecture.md)**: システム設計と構成要素
- **[🚀 デプロイメントガイド](./docs/deployment-guide.md)**: 詳細なデプロイ手順
- **[💻 開発ガイド](./docs/development-guide.md)**: 開発ワークフローとベストプラクティス
- **[🧩 構成要素](./docs/constructs.md)**: CDK構成要素の詳細仕様

### 🔧 運用ガイド
- **環境管理**: 開発・ステージング・本番環境の管理
- **セキュリティ**: IAMロール、セキュリティヘッダー、SSL証明書
- **モニタリング**: CloudWatchログとメトリクス
- **トラブルシューティング**: よくある問題と解決方法

## 💰 コスト

このプロジェクトで使用するAWSリソースの概算コストです：

### 主要リソース
- **CloudFront**: 月額約$1-5（トラフィック量による）
- **S3**: 月額約$0.50-2（ストレージ量による）
- **Route 53**: ホストゾーン月額$0.50 + クエリ料金
- **ACM証明書**: 無料

### 無料利用枠対象外
- Route 53ホストゾーン料金
- CloudFrontのデータ転送料金（1TBを超える場合）

⚠️ **注意**: 実際のコストはトラフィック量、ストレージ使用量、リージョンによって変動します。[AWS料金計算ツール](https://calculator.aws/)で詳細な見積もりを確認してください。

## 🔐 セキュリティ

- **最小権限の原則**: 必要最小限のIAM権限のみ付与
- **セキュリティヘッダー**: CSP、HSTS等の適切な設定
- **SSL/TLS**: 全通信のHTTPS化
- **アクセス制御**: CloudFrontでの適切なアクセス制御

## 🤝 開発ワークフロー

1. **機能ブランチ作成**: `git checkout -b feature/cdk-[feature-name]`
2. **開発**: コード作成・テスト・ビルド
3. **品質チェック**: TypeScriptコンパイル・テスト・CDK構文チェック
4. **デプロイテスト**: 開発環境での動作確認
5. **プルリクエスト**: レビュー後にマージ

## 🔗 関連リンク

- **[プロジェクト概要](../README.md)**: WebToolsプロジェクト全体の概要
- **[Webアプリケーション](../web/README.md)**: フロントエンド開発ガイド
- **[開発ガイドライン](../.junie/guidelines.md)**: 包括的な開発・運用ガイドライン
- **[AWS CDK公式ドキュメント](https://docs.aws.amazon.com/cdk/)**: AWS CDKの公式リファレンス
