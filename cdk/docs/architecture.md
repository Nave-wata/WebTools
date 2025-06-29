# CDK アーキテクチャ概要

このドキュメントでは、WebTools プロジェクトのCDKアーキテクチャについて詳細に説明します。

## 全体アーキテクチャ

WebTools のインフラストラクチャは、静的サイトホスティングのための3つの主要なスタックで構成されています：

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   DomainStack   │    │  StorageStack   │    │    CdnStack     │
│                 │    │                 │    │                 │
│ • Route53       │    │ • S3 Bucket     │    │ • CloudFront    │
│ • SSL証明書     │    │ • バケット設定  │    │ • DNS レコード  │
│ • ホストゾーン  │    │ • 静的サイト    │    │ • セキュリティ  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                    ┌─────────────────┐
                    │   依存関係      │
                    │                 │
                    │ CdnStack は     │
                    │ 他の2つに依存   │
                    └─────────────────┘
```

## スタック詳細

### 1. DomainStack

**目的**: ドメイン管理とSSL証明書の提供

**リージョン**: us-east-1 (CloudFrontの要件)

**主要リソース**:
- Route53 ホストゾーン参照
- ACM SSL証明書 (CloudFront用)

**出力**:
- `certificate`: SSL証明書のARN

### 2. StorageStack

**目的**: 静的サイトファイルの保存

**リージョン**: 設定可能 (環境変数)

**主要リソース**:
- S3バケット (静的サイトホスティング設定)
- バケットポリシー (CloudFrontからのアクセス許可)

**出力**:
- `bucket`: S3バケットの参照

### 3. CdnStack

**目的**: コンテンツ配信とDNS設定

**リージョン**: 設定可能 (環境変数)

**主要リソース**:
- CloudFrontディストリビューション
- Route53 DNSレコード
- セキュリティヘッダー設定

**依存関係**:
- DomainStack (SSL証明書)
- StorageStack (S3バケット)

## 構成要素 (Constructs) アーキテクチャ

### ドメイン別構成

```
lib/constructs/
├── cdn/                    # CloudFront関連
│   ├── cloudfront.ts      # メインディストリビューション
│   ├── security-headers-construct.ts  # セキュリティヘッダー
│   ├── cloudfront-function.ts         # エッジ関数
│   └── bucket-policy-custom-resource.ts # バケットポリシー
├── domain/                 # ドメイン・証明書関連
│   ├── certificate.ts     # SSL証明書管理
│   └── dns.ts            # DNS設定
├── shared/                 # 共通構成要素
│   ├── host-zone.ts      # ホストゾーン参照
│   └── bucket-reference.ts # バケット参照
└── storage/                # ストレージ関連
    └── static-site.ts     # 静的サイト設定
```

### 型定義アーキテクチャ

```
lib/interfaces/
├── shared.ts              # 共通型定義
├── domain.ts              # ドメイン関連型
├── storage.ts             # ストレージ関連型
└── cdn.ts                 # CDN関連型
```

## 設計原則

### 1. 単一責任の原則

各スタックは明確に定義された単一の責任を持ちます：
- **DomainStack**: ドメインとSSL証明書の管理のみ
- **StorageStack**: ストレージの設定のみ
- **CdnStack**: コンテンツ配信とDNS設定のみ

### 2. 依存関係の明示化

```typescript
// 依存関係を明示的に指定
cdnStack.addDependency(domainStack);
cdnStack.addDependency(storageStack);
```

### 3. クロスリージョン対応

```typescript
// クロスリージョンの参照を有効にする
crossRegionReferences: true
```

### 4. 環境分離

```typescript
// 環境変数による設定の外部化
const stackProps = {
  env: {
    account: requireEnv("AWS_DEFAULT_ACCOUNT"),
    region: requireEnv("AWS_DEFAULT_REGION"),
  },
};
```

## セキュリティアーキテクチャ

### 1. アクセス制御

- S3バケットは直接アクセスを禁止
- CloudFrontからのみアクセス可能
- OAI (Origin Access Identity) による制御

### 2. SSL/TLS

- 全通信をHTTPS化
- ACM証明書による暗号化
- セキュリティヘッダーの自動付与

### 3. セキュリティヘッダー

CloudFront Functionによる自動付与：
- `Strict-Transport-Security`
- `X-Content-Type-Options`
- `X-Frame-Options`
- `X-XSS-Protection`
- `Referrer-Policy`

## パフォーマンス最適化

### 1. CloudFront設定

- グローバルエッジロケーション活用
- 適切なキャッシュ設定
- 圧縮の有効化

### 2. S3最適化

- 適切なストレージクラス設定
- 静的サイトホスティング最適化

## 監視とログ

### 1. CloudWatch統合

- CloudFrontアクセスログ
- S3アクセスログ
- エラー監視

### 2. コスト最適化

- 適切なリソース設定
- 不要なリソースの自動削除

## 拡張性

### 1. 新機能追加

構成要素ベースの設計により、新機能を容易に追加可能：
- 新しい構成要素の作成
- 既存スタックへの統合
- 型安全性の維持

### 2. 環境対応

環境変数による設定により、複数環境への対応が容易：
- 開発環境
- ステージング環境
- 本番環境

## トラブルシューティング

### 1. 一般的な問題

- クロスリージョン参照エラー
- 依存関係の循環参照
- 環境変数の未設定

### 2. デバッグ方法

```bash
# 構文チェック
npx cdk synth

# 差分確認
npx cdk diff

# デプロイ前確認
npx cdk deploy --dry-run
```

詳細な実装については、各構成要素のドキュメントを参照してください。
