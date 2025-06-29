# CDK 開発ガイド

このドキュメントでは、WebTools プロジェクトのCDK開発における開発ワークフロー、テスト方法、ベストプラクティスについて説明します。

## 開発環境のセットアップ

### 1. 必要なツール

```bash
# Node.js (v18以上)
node --version

# npm または yarn
npm --version

# AWS CDK CLI
npm install -g aws-cdk
cdk --version

# TypeScript
npm install -g typescript
tsc --version
```

### 2. プロジェクトの初期化

```bash
# リポジトリのクローン
git clone <repository-url>
cd WebTools/cdk

# 依存関係のインストール
npm install

# TypeScriptのコンパイル
npm run build

# テストの実行
npm test
```

### 3. IDE設定

#### Visual Studio Code

推奨拡張機能：
- TypeScript and JavaScript Language Features
- AWS Toolkit
- ESLint
- Prettier

設定ファイル (`.vscode/settings.json`):
```json
{
  "typescript.preferences.importModuleSpecifier": "relative",
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll.eslint": true
  }
}
```

## 開発ワークフロー

### 1. 機能開発の流れ

```bash
# 1. 新しいブランチの作成
git checkout -b feature/new-construct

# 2. 開発作業
# - 型定義の作成 (interfaces/)
# - 構成要素の実装 (constructs/)
# - スタックへの統合 (stacks/)
# - テストの作成 (test/)

# 3. コンパイルとテスト
npm run build
npm test

# 4. CDK構文チェック
npx cdk synth

# 5. 差分確認
npx cdk diff

# 6. コミットとプッシュ
git add .
git commit -m "feat: add new construct for ..."
git push origin feature/new-construct
```

### 2. コード品質の確保

#### TypeScriptコンパイル

```bash
# コンパイル
npm run build

# ウォッチモード
npm run watch

# 型チェックのみ
npx tsc --noEmit
```

#### リンターとフォーマッター

```bash
# ESLintの実行
npm run lint

# 自動修正
npm run lint:fix

# Prettierでのフォーマット
npm run format
```

#### テストの実行

```bash
# 全テストの実行
npm test

# ウォッチモード
npm run test:watch

# カバレッジレポート
npm run test:coverage

# 特定のテストファイル
npm test -- --testNamePattern="DomainStack"
```

## テスト戦略

### 1. ユニットテスト

#### 構成要素のテスト

```typescript
import { Template } from 'aws-cdk-lib/assertions';
import { Stack } from 'aws-cdk-lib';
import { SecurityHeadersConstruct } from '../../lib/constructs/cdn/security-headers-construct';

describe('SecurityHeadersConstruct', () => {
  test('creates CloudFront function with security headers', () => {
    const stack = new Stack();

    new SecurityHeadersConstruct(stack, 'TestSecurityHeaders');

    const template = Template.fromStack(stack);

    template.hasResourceProperties('AWS::CloudFront::Function', {
      FunctionConfig: {
        Comment: 'Security headers function',
        Runtime: 'cloudfront-js-1.0'
      }
    });
  });
});
```

#### スタックのテスト

```typescript
import { Template } from 'aws-cdk-lib/assertions';
import { App } from 'aws-cdk-lib';
import { DomainStack } from '../../lib/stacks/domain';

describe('DomainStack', () => {
  test('creates ACM certificate', () => {
    const app = new App();
    const stack = new DomainStack(app, 'TestDomainStack', {
      zoneName: 'example.com',
      domainName: 'test.example.com',
    });

    const template = Template.fromStack(stack);

    template.hasResourceProperties('AWS::CertificateManager::Certificate', {
      DomainName: 'test.example.com',
      ValidationMethod: 'DNS'
    });
  });
});
```

### 2. 統合テスト

```typescript
describe('Full Stack Integration', () => {
  test('all stacks can be synthesized together', () => {
    const app = new App();

    const domainStack = new DomainStack(app, 'TestDomainStack', {
      zoneName: 'example.com',
      domainName: 'test.example.com',
    });

    const storageStack = new StorageStack(app, 'TestStorageStack', {
      domainName: 'test.example.com',
    });

    const cdnStack = new CdnStack(app, 'TestCdnStack', {
      domainName: 'test.example.com',
      zoneName: 'example.com',
      recordName: 'test',
      certificate: domainStack.certificate,
      bucket: storageStack.bucket,
    });

    // 依存関係の設定
    cdnStack.addDependency(domainStack);
    cdnStack.addDependency(storageStack);

    // 全スタックが正常にシンセサイズできることを確認
    expect(() => app.synth()).not.toThrow();
  });
});
```

### 3. スナップショットテスト

```typescript
test('DomainStack snapshot', () => {
  const app = new App();
  const stack = new DomainStack(app, 'TestDomainStack', {
    zoneName: 'example.com',
    domainName: 'test.example.com',
  });

  const template = Template.fromStack(stack);
  expect(template.toJSON()).toMatchSnapshot();
});
```

## デバッグとトラブルシューティング

### 1. ローカルデバッグ

#### CDK構文チェック

```bash
# 全スタックの構文チェック
npx cdk synth

# 特定スタックのみ
npx cdk synth ToolsNaveWataNet-DomainStack

# 出力をファイルに保存
npx cdk synth > cloudformation-template.yaml
```

#### 差分確認

```bash
# 全スタックの差分
npx cdk diff

# 特定スタックの差分
npx cdk diff ToolsNaveWataNet-CdnStack

# 詳細な差分表示
npx cdk diff --verbose
```

### 2. ログとデバッグ情報

```bash
# CDKデバッグモード
export CDK_DEBUG=true
npx cdk synth

# AWS CLIデバッグモード
export AWS_CLI_DEBUG=true
aws cloudformation describe-stacks --stack-name ToolsNaveWataNet-DomainStack

# CloudFormationイベントの監視
aws cloudformation describe-stack-events --stack-name ToolsNaveWataNet-CdnStack
```

### 3. 一般的な問題と解決方法

#### TypeScriptコンパイルエラー

```bash
# 型定義の確認
npm run build 2>&1 | grep -E "(error|Error)"

# 依存関係の更新
npm update

# node_modulesの再インストール
rm -rf node_modules package-lock.json
npm install
```

#### CDK構文エラー

```bash
# 詳細なエラー情報
npx cdk synth --verbose

# 特定の構成要素のテスト
npm test -- --testNamePattern="YourConstruct"
```

## ベストプラクティス

### 1. コード構成

#### ディレクトリ構造

```
lib/
├── constructs/          # 再利用可能な構成要素
│   ├── domain/         # ドメイン別に分類
│   ├── storage/
│   └── cdn/
├── interfaces/          # 型定義
├── stacks/             # スタック定義
└── utils/              # ユーティリティ関数
```

#### 命名規則

```typescript
// スタック名: パスカルケース + Stack
export class DomainStack extends Stack {}

// 構成要素名: パスカルケース + Construct
export class SecurityHeadersConstruct extends Construct {}

// インターフェース名: パスカルケース + Props
export interface DomainStackProps extends StackProps {}

// 変数・関数: キャメルケース
const domainName = 'example.com';
function createCertificate() {}
```

### 2. 型安全性

#### 厳密な型定義

```typescript
// 良い例: 厳密な型定義
export interface CdnStackProps extends StackProps {
  readonly domainName: string;
  readonly zoneName: string;
  readonly recordName: string;
  readonly certificate: ICertificate;
  readonly bucket: IBucket;
}

// 悪い例: any型の使用
export interface CdnStackProps {
  [key: string]: any;
}
```

#### 型ガード

```typescript
function isCertificate(obj: any): obj is ICertificate {
  return obj && typeof obj.certificateArn === 'string';
}

if (isCertificate(props.certificate)) {
  // 型安全な操作
}
```

### 3. エラーハンドリング

#### 環境変数の検証

```typescript
export function requireEnv(name: string): string {
  const value = process.env[name];

  if (!value) {
    throw new Error(`環境変数 ${name} が設定されていません`);
  }

  return value;
}
```

#### 構成要素の検証

```typescript
export class DomainStack extends Stack {
  constructor(scope: Construct, id: string, props: DomainStackProps) {
    super(scope, id, props);

    // 入力値の検証
    if (!props.domainName) {
      throw new Error('domainName is required');
    }

    if (!props.zoneName) {
      throw new Error('zoneName is required');
    }
  }
}
```

### 4. パフォーマンス最適化

#### 構成要素の再利用

```typescript
// 良い例: 再利用可能な構成要素
export class HostZoneConstruct extends Construct {
  public readonly hostedZone: IHostedZone;

  constructor(scope: Construct, id: string, props: HostZoneProps) {
    super(scope, id);

    this.hostedZone = HostedZone.fromLookup(this, 'HostedZone', {
      domainName: props.zoneName,
    });
  }
}

// 悪い例: 重複した実装
// 各スタックで同じHostedZone.fromLookupを繰り返す
```

#### 条件付きリソース作成

```typescript
// 開発環境でのみ作成するリソース
if (props.environment === 'development') {
  new CloudWatchDashboard(this, 'DevDashboard', {
    // 開発用ダッシュボード設定
  });
}
```

### 5. セキュリティ

#### 最小権限の原則

```typescript
// 良い例: 必要最小限の権限
const restrictedBucketPolicy = new PolicyStatement({
  effect: Effect.ALLOW,
  principals: [new ServicePrincipal('cloudfront.amazonaws.com')],
  actions: ['s3:GetObject'],
  resources: [`${bucket.bucketArn}/*`],
});

// 悪い例: 過度な権限
const permissiveBucketPolicy = new PolicyStatement({
  effect: Effect.ALLOW,
  principals: [new AnyPrincipal()],
  actions: ['s3:*'],
  resources: ['*'],
});
```

#### 機密情報の管理

```typescript
// 良い例: 環境変数の使用
const secureApiKey = requireEnv('API_KEY');

// 悪い例: ハードコーディング
const hardcodedApiKey = 'secret-api-key-123';
```

## CI/CD統合

### 1. GitHub Actions

```yaml
name: CDK CI/CD

on:
  push:
    branches: [main]
    paths: ['cdk/**']
  pull_request:
    branches: [main]
    paths: ['cdk/**']

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'
          cache: 'npm'
          cache-dependency-path: cdk/package-lock.json

      - name: Install dependencies
        run: |
          cd cdk
          npm ci

      - name: Run tests
        run: |
          cd cdk
          npm test

      - name: CDK Synth
        run: |
          cd cdk
          npx cdk synth
```

### 2. デプロイパイプライン

```yaml
  deploy:
    needs: test
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    steps:
      - name: Configure AWS credentials
        uses: aws-actions/configure-aws-credentials@v2
        with:
          aws-access-key-id: ${{ secrets.AWS_ACCESS_KEY_ID }}
          aws-secret-access-key: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
          aws-region: ap-northeast-1

      - name: Deploy CDK
        run: |
          cd cdk
          npx cdk deploy --all --require-approval never
```

## 監視とメンテナンス

### 1. CloudWatchメトリクス

```typescript
// カスタムメトリクスの追加
new Metric({
  namespace: 'WebTools/CDK',
  metricName: 'DeploymentCount',
  dimensionsMap: {
    Stack: this.stackName,
  },
});
```

### 2. アラート設定

```typescript
// CloudWatchアラーム
new Alarm(this, 'HighErrorRate', {
  metric: distribution.metricErrorRate(),
  threshold: 5,
  evaluationPeriods: 2,
});
```

### 3. 定期的なメンテナンス

```bash
# 依存関係の更新
npm update

# セキュリティ監査
npm audit

# 未使用の依存関係の確認
npx depcheck
```

詳細な実装については、[architecture.md](./architecture.md) と [deployment-guide.md](./deployment-guide.md) を参照してください。
