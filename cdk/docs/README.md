# CDK ドキュメント

WebTools プロジェクトのAWS CDK (Cloud Development Kit) に関する包括的なドキュメントです。

## 📚 ドキュメント一覧

### [🏗️ アーキテクチャ概要](./architecture.md)
CDKプロジェクトの全体アーキテクチャ、設計原則、セキュリティ考慮事項について詳細に説明しています。

**主な内容:**
- 3つのメインスタック (Domain, Storage, CDN) の構成
- 構成要素のドメイン別分類
- セキュリティアーキテクチャ
- パフォーマンス最適化
- 拡張性とメンテナンス

### [🚀 デプロイメントガイド](./deployment-guide.md)
CDKスタックのデプロイ方法、環境設定、トラブルシューティングについて説明しています。

**主な内容:**
- 前提条件と環境設定
- 段階的デプロイ手順
- デプロイ後の確認方法
- ファイルアップロードとキャッシュ無効化
- 更新とメンテナンス
- トラブルシューティング

### [💻 開発ガイド](./development-guide.md)
CDK開発における開発ワークフロー、テスト方法、ベストプラクティスについて説明しています。

**主な内容:**
- 開発環境のセットアップ
- 開発ワークフローとコード品質管理
- テスト戦略 (ユニット、統合、スナップショット)
- デバッグとトラブルシューティング
- ベストプラクティス
- CI/CD統合

### [🧩 構成要素ドキュメント](./constructs.md)
個々のCDK構成要素の詳細な仕様、使用方法、型定義について説明しています。

**主な内容:**
- Domain構成要素 (Certificate, DNS)
- Storage構成要素 (StaticSite)
- CDN構成要素 (CloudFront, SecurityHeaders)
- Shared構成要素 (HostZone, BucketReference)
- 型定義とインターフェース
- 使用パターンとベストプラクティス

## 🚀 クイックスタート

### 1. 環境準備

```bash
# CDKディレクトリに移動
cd cdk

# 依存関係のインストール
npm install

# 環境変数の設定
cp .env.example .env
# .envファイルを編集して必要な値を設定
```

### 2. 開発・テスト

```bash
# TypeScriptのコンパイル
npm run build

# テストの実行
npm test

# CDK構文チェック
npx cdk synth
```

### 3. デプロイ

```bash
# 差分確認
npx cdk diff

# 全スタックのデプロイ
npx cdk deploy --all
```

## 📋 プロジェクト構造

```
/cdk
├── bin/                  # CDKアプリケーションのエントリーポイント
│   └── app.ts           # メインアプリケーション定義
├── lib/                 # CDK構成要素の実装
│   ├── constructs/      # 再利用可能なCDK構成要素
│   │   ├── cdn/         # CloudFront関連の構成要素
│   │   ├── domain/      # ドメイン・証明書関連の構成要素
│   │   ├── shared/      # 共通の構成要素
│   │   └── storage/     # ストレージ関連の構成要素
│   ├── interfaces/      # TypeScript型定義
│   └── stacks/          # CDKスタック定義
├── test/                # テストファイル
├── utils/               # ユーティリティ関数
├── assets/              # デプロイ用アセット
├── docs/                # このドキュメント
├── .env                 # 環境変数設定
└── .env.example         # 環境変数テンプレート
```

## 🔧 主要なスタック

### DomainStack
- **目的**: ドメイン管理とSSL証明書の提供
- **リージョン**: us-east-1 (CloudFrontの要件)
- **主要リソース**: Route53ホストゾーン、ACM SSL証明書

### StorageStack
- **目的**: 静的サイトファイルの保存
- **リージョン**: 設定可能
- **主要リソース**: S3バケット、バケットポリシー

### CdnStack
- **目的**: コンテンツ配信とDNS設定
- **リージョン**: 設定可能
- **主要リソース**: CloudFrontディストリビューション、Route53 DNSレコード

## 🛡️ セキュリティ

### 実装されているセキュリティ機能

- **HTTPS強制**: 全通信をHTTPS化
- **セキュリティヘッダー**: CloudFront Functionによる自動付与
  - `Strict-Transport-Security`
  - `X-Content-Type-Options`
  - `X-Frame-Options`
  - `X-XSS-Protection`
  - `Referrer-Policy`
- **アクセス制御**: S3バケットの直接アクセス禁止
- **最小権限**: IAMロールとポリシーの最小権限設定

### セキュリティベストプラクティス

- 環境変数による機密情報管理
- 定期的なセキュリティ監査
- CloudTrailによるAPI呼び出し記録
- CloudWatchによる監視

## 📊 監視とメンテナンス

### 監視項目

- CloudFrontアクセスログ
- S3アクセスログ
- CloudWatchメトリクス
- エラー率とレスポンス時間

### 定期メンテナンス

```bash
# 依存関係の更新
npm update

# セキュリティ監査
npm audit

# 未使用の依存関係の確認
npx depcheck
```

## 🔗 関連リンク

### AWS公式ドキュメント
- [AWS CDK Developer Guide](https://docs.aws.amazon.com/cdk/v2/guide/)
- [AWS CloudFront Documentation](https://docs.aws.amazon.com/cloudfront/)
- [AWS S3 Documentation](https://docs.aws.amazon.com/s3/)
- [AWS Route53 Documentation](https://docs.aws.amazon.com/route53/)

### CDKリソース
- [AWS CDK API Reference](https://docs.aws.amazon.com/cdk/api/v2/)
- [CDK Patterns](https://cdkpatterns.com/)
- [AWS CDK Examples](https://github.com/aws-samples/aws-cdk-examples)

## 🤝 コントリビューション

### 開発ワークフロー

1. **ブランチ作成**: `git checkout -b feature/new-feature`
2. **開発**: 構成要素の実装とテスト作成
3. **品質チェック**: `npm run build && npm test && npx cdk synth`
4. **プルリクエスト**: レビューとマージ

### コーディング規約

- **命名規則**: パスカルケース (スタック、構成要素)、キャメルケース (変数、関数)
- **型安全性**: TypeScriptの型システムを活用
- **コメント**: 日本語での詳細なコメント
- **テスト**: 新機能には必ずテストを追加

## 📞 サポート

### トラブルシューティング

一般的な問題と解決方法については、各ドキュメントのトラブルシューティングセクションを参照してください：

- [デプロイメント関連](./deployment-guide.md#トラブルシューティング)
- [開発関連](./development-guide.md#デバッグとトラブルシューティング)
- [構成要素関連](./constructs.md#トラブルシューティング)

### デバッグ方法

```bash
# CDKデバッグモード
export CDK_DEBUG=true
npx cdk synth

# 詳細ログの確認
npx cdk synth --verbose

# CloudFormationイベントの監視
aws cloudformation describe-stack-events --stack-name <stack-name>
```

---

**最終更新**: 2024年6月

このドキュメントは、WebTools プロジェクトのCDK実装に関する包括的なガイドです。質問や改善提案がある場合は、プロジェクトのIssueまたはプルリクエストを作成してください。
