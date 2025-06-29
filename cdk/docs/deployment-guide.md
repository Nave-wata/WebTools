# CDK デプロイメントガイド

このドキュメントでは、WebTools プロジェクトのCDKスタックのデプロイ方法について詳細に説明します。

## 前提条件

### 1. 必要なツール

- Node.js (v18以上)
- npm または yarn
- AWS CLI v2
- AWS CDK CLI

### 2. AWS認証情報の設定

```bash
# AWS CLIの設定
aws configure

# または環境変数で設定
export AWS_ACCESS_KEY_ID=your-access-key
export AWS_SECRET_ACCESS_KEY=your-secret-key
export AWS_DEFAULT_REGION=ap-northeast-1
```

### 3. 必要なAWS権限

デプロイには以下のAWSサービスへの権限が必要です：

- **Route53**: ホストゾーンとDNSレコードの管理
- **ACM**: SSL証明書の作成と管理
- **S3**: バケットの作成と設定
- **CloudFront**: ディストリビューションの作成と管理
- **IAM**: ロールとポリシーの作成
- **CloudFormation**: スタックの管理

## 環境設定

### 1. プロジェクトのセットアップ

```bash
# CDKディレクトリに移動
cd cdk

# 依存関係のインストール
npm install

# TypeScriptのコンパイル
npm run build
```

### 2. 環境変数の設定

```bash
# 環境変数ファイルをコピー
cp .env.example .env

# .envファイルを編集
vim .env
```

必要な環境変数：

```bash
# AWSアカウント情報
AWS_DEFAULT_ACCOUNT=123456789012
AWS_DEFAULT_REGION=ap-northeast-1

# ドメイン設定
DOMAIN_NAME=tools.nave-wata.net
ZONE_NAME=nave-wata.net
RECORD_NAME=tools
```

### 3. CDKの初期化

```bash
# CDKのブートストラップ（初回のみ）
npx cdk bootstrap aws://123456789012/us-east-1
npx cdk bootstrap aws://123456789012/ap-northeast-1
```

## デプロイ手順

### 1. 事前確認

```bash
# 構文チェック
npx cdk synth

# デプロイ対象の確認
npx cdk list

# 差分の確認
npx cdk diff
```

### 2. 段階的デプロイ

#### Step 1: ドメインスタックのデプロイ

```bash
# DomainStackのデプロイ
npx cdk deploy ToolsNaveWataNet-DomainStack

# デプロイ状況の確認
aws cloudformation describe-stacks --stack-name ToolsNaveWataNet-DomainStack
```

**注意**: SSL証明書の検証には時間がかかる場合があります（最大30分）。

#### Step 2: ストレージスタックのデプロイ

```bash
# StorageStackのデプロイ
npx cdk deploy ToolsNaveWataNet-StorageStack

# S3バケットの確認
aws s3 ls | grep tools-nave-wata-net
```

#### Step 3: CDNスタックのデプロイ

```bash
# CdnStackのデプロイ
npx cdk deploy ToolsNaveWataNet-CdnStack

# CloudFrontディストリビューションの確認
aws cloudfront list-distributions
```

### 3. 全スタック一括デプロイ

```bash
# 全スタックを一括でデプロイ
npx cdk deploy --all

# 確認プロンプトをスキップする場合
npx cdk deploy --all --require-approval never
```

## デプロイ後の確認

### 1. リソースの確認

```bash
# デプロイされたスタックの一覧
aws cloudformation list-stacks --stack-status-filter CREATE_COMPLETE UPDATE_COMPLETE

# 各スタックの詳細確認
aws cloudformation describe-stacks --stack-name ToolsNaveWataNet-DomainStack
aws cloudformation describe-stacks --stack-name ToolsNaveWataNet-StorageStack
aws cloudformation describe-stacks --stack-name ToolsNaveWataNet-CdnStack
```

### 2. 動作確認

```bash
# DNSの確認
nslookup tools.nave-wata.net

# HTTPSアクセスの確認
curl -I https://tools.nave-wata.net

# セキュリティヘッダーの確認
curl -I https://tools.nave-wata.net | grep -E "(Strict-Transport-Security|X-Content-Type-Options)"
```

### 3. ログの確認

```bash
# CloudFormationイベントの確認
aws cloudformation describe-stack-events --stack-name ToolsNaveWataNet-CdnStack

# CloudFrontログの確認（設定されている場合）
aws logs describe-log-groups --log-group-name-prefix /aws/cloudfront
```

## ファイルのアップロード

### 1. 手動アップロード

```bash
# S3バケットへのファイルアップロード
aws s3 sync ./dist s3://your-bucket-name --delete

# キャッシュの無効化
aws cloudfront create-invalidation --distribution-id YOUR_DISTRIBUTION_ID --paths "/*"
```

### 2. 自動化スクリプト

```bash
#!/bin/bash
# deploy-site.sh

BUCKET_NAME=$(aws cloudformation describe-stacks \
  --stack-name ToolsNaveWataNet-StorageStack \
  --query 'Stacks[0].Outputs[?OutputKey==`BucketName`].OutputValue' \
  --output text)

DISTRIBUTION_ID=$(aws cloudformation describe-stacks \
  --stack-name ToolsNaveWataNet-CdnStack \
  --query 'Stacks[0].Outputs[?OutputKey==`DistributionId`].OutputValue' \
  --output text)

# ファイルのアップロード
aws s3 sync ./dist s3://$BUCKET_NAME --delete

# キャッシュの無効化
aws cloudfront create-invalidation --distribution-id $DISTRIBUTION_ID --paths "/*"

echo "デプロイが完了しました"
```

## 更新とメンテナンス

### 1. スタックの更新

```bash
# 変更の確認
npx cdk diff

# 特定スタックの更新
npx cdk deploy ToolsNaveWataNet-CdnStack

# 全スタックの更新
npx cdk deploy --all
```

### 2. 設定変更

環境変数やドメイン設定を変更した場合：

```bash
# .envファイルを更新
vim .env

# 変更を反映
npm run build
npx cdk diff
npx cdk deploy --all
```

### 3. ロールバック

```bash
# CloudFormationでのロールバック
aws cloudformation cancel-update-stack --stack-name ToolsNaveWataNet-CdnStack

# 特定バージョンへの復元
aws cloudformation update-stack --stack-name ToolsNaveWataNet-CdnStack --use-previous-template
```

## トラブルシューティング

### 1. 一般的なエラー

#### SSL証明書の検証エラー

```bash
# 証明書の状態確認
aws acm list-certificates --region us-east-1

# DNS検証レコードの確認
aws route53 list-resource-record-sets --hosted-zone-id YOUR_ZONE_ID
```

#### クロスリージョン参照エラー

```bash
# 依存関係の確認
npx cdk synth | grep -A 10 -B 10 "cross-region"

# 明示的な依存関係の追加
cdnStack.addDependency(domainStack);
```

#### 権限エラー

```bash
# 現在のIAMユーザー/ロールの確認
aws sts get-caller-identity

# 必要な権限の確認
aws iam simulate-principal-policy --policy-source-arn YOUR_USER_ARN --action-names cloudformation:CreateStack
```

### 2. デバッグ方法

```bash
# 詳細ログの有効化
export CDK_DEBUG=true

# CloudFormationテンプレートの出力
npx cdk synth > template.yaml

# スタックイベントの監視
aws cloudformation describe-stack-events --stack-name ToolsNaveWataNet-CdnStack --query 'StackEvents[0:10]'
```

### 3. 緊急時の対応

#### 完全な削除と再作成

```bash
# 全スタックの削除（注意：データが失われます）
npx cdk destroy --all

# 再デプロイ
npx cdk deploy --all
```

#### 部分的な復旧

```bash
# 特定リソースのみ削除
aws cloudformation delete-stack --stack-name ToolsNaveWataNet-CdnStack

# 依存関係を考慮した再デプロイ
npx cdk deploy ToolsNaveWataNet-CdnStack
```

## セキュリティ考慮事項

### 1. 認証情報の管理

- AWS認証情報をコードに含めない
- 環境変数や AWS Secrets Manager を使用
- IAMロールの最小権限の原則を適用

### 2. ネットワークセキュリティ

- S3バケットの直接アクセスを禁止
- CloudFrontでのHTTPS強制
- 適切なセキュリティヘッダーの設定

### 3. 監査とログ

- CloudTrailでのAPI呼び出し記録
- CloudWatchでのメトリクス監視
- 定期的なセキュリティ監査

## コスト最適化

### 1. リソースの最適化

```bash
# 使用されていないリソースの確認
aws cloudformation describe-stacks --query 'Stacks[?StackStatus==`DELETE_COMPLETE`]'

# CloudFrontの使用量確認
aws cloudwatch get-metric-statistics --namespace AWS/CloudFront --metric-name Requests
```

### 2. 定期的な見直し

- 月次でのコスト分析
- 不要なリソースの削除
- 適切なストレージクラスの選択

詳細な実装については、[architecture.md](./architecture.md) を参照してください。
