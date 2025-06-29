# CDK 構成要素 (Constructs) ドキュメント

このドキュメントでは、WebTools プロジェクトで使用されているCDK構成要素について詳細に説明します。

## 構成要素の概要

WebTools プロジェクトの構成要素は、機能ごとにドメイン別に分類されています：

- **Domain**: ドメインとSSL証明書関連
- **Storage**: S3ストレージ関連
- **CDN**: CloudFrontとコンテンツ配信関連
- **Shared**: 複数のドメインで共有される構成要素

## Domain 構成要素

### CertificateConstruct

**ファイル**: `lib/constructs/domain/certificate.ts`

**目的**: SSL証明書の作成と管理

#### 機能
- ACM (AWS Certificate Manager) 証明書の作成
- DNS検証の設定
- CloudFront用の証明書設定 (us-east-1リージョン)

#### 使用例
```typescript
import { CertificateConstruct } from '../constructs/domain/certificate';

const certificate = new CertificateConstruct(this, 'Certificate', {
  domainName: 'tools.nave-wata.net',
  hostedZone: hostedZone,
});
```

#### プロパティ
- `domainName`: 証明書を発行するドメイン名
- `hostedZone`: DNS検証用のRoute53ホストゾーン

#### 出力
- `certificate`: 作成されたACM証明書のインスタンス

### DnsConstruct

**ファイル**: `lib/constructs/domain/dns.ts`

**目的**: DNS設定とRoute53レコードの管理

#### 機能
- Route53 Aレコードの作成
- CloudFrontディストリビューションへのエイリアス設定
- IPv6対応 (AAAAレコード)

#### 使用例
```typescript
import { DnsConstruct } from '../constructs/domain/dns';

const dns = new DnsConstruct(this, 'Dns', {
  hostedZone: hostedZone,
  recordName: 'tools',
  target: distribution,
});
```

#### プロパティ
- `hostedZone`: Route53ホストゾーン
- `recordName`: 作成するレコード名 (サブドメイン)
- `target`: エイリアス先のCloudFrontディストリビューション

## Storage 構成要素

### StaticSiteConstruct

**ファイル**: `lib/constructs/storage/static-site.ts`

**目的**: 静的サイトホスティング用のS3バケット設定

#### 機能
- S3バケットの作成
- 静的サイトホスティングの設定
- パブリックアクセスブロックの設定
- CloudFront用のバケットポリシー設定

#### 使用例
```typescript
import { StaticSiteConstruct } from '../constructs/storage/static-site';

const staticSite = new StaticSiteConstruct(this, 'StaticSite', {
  domainName: 'tools.nave-wata.net',
  bucketName: 'tools-nave-wata-net-static-site',
});
```

#### プロパティ
- `domainName`: 関連するドメイン名
- `bucketName`: S3バケット名 (オプション)

#### 出力
- `bucket`: 作成されたS3バケットのインスタンス
- `bucketDomainName`: バケットのドメイン名

## CDN 構成要素

### CloudFrontConstruct

**ファイル**: `lib/constructs/cdn/cloudfront.ts`

**目的**: CloudFrontディストリビューションの作成と設定

#### 機能
- CloudFrontディストリビューションの作成
- S3オリジンの設定
- キャッシュビヘイビアの設定
- SSL証明書の関連付け
- セキュリティヘッダーの設定

#### 使用例
```typescript
import { CloudFrontConstruct } from '../constructs/cdn/cloudfront';

const cloudfront = new CloudFrontConstruct(this, 'CloudFront', {
  domainName: 'tools.nave-wata.net',
  certificate: certificate,
  bucket: bucket,
  securityHeadersFunction: securityHeadersFunction,
});
```

#### プロパティ
- `domainName`: CloudFrontで使用するドメイン名
- `certificate`: SSL証明書
- `bucket`: オリジンとなるS3バケット
- `securityHeadersFunction`: セキュリティヘッダー用のCloudFront Function

#### 出力
- `distribution`: 作成されたCloudFrontディストリビューション
- `distributionId`: ディストリビューションID
- `distributionDomainName`: ディストリビューションのドメイン名

### SecurityHeadersConstruct

**ファイル**: `lib/constructs/cdn/security-headers-construct.ts`

**目的**: セキュリティヘッダーを自動付与するCloudFront Functionの作成

#### 機能
- CloudFront Functionの作成
- セキュリティヘッダーの自動付与
- レスポンス改変処理

#### セキュリティヘッダー
- `Strict-Transport-Security`: HTTPS強制
- `X-Content-Type-Options`: MIMEタイプスニッフィング防止
- `X-Frame-Options`: クリックジャッキング防止
- `X-XSS-Protection`: XSS攻撃防止
- `Referrer-Policy`: リファラー情報制御

#### 使用例
```typescript
import { SecurityHeadersConstruct } from '../constructs/cdn/security-headers-construct';

const securityHeaders = new SecurityHeadersConstruct(this, 'SecurityHeaders');
```

#### 出力
- `function`: 作成されたCloudFront Function

### CloudFrontFunctionConstruct

**ファイル**: `lib/constructs/cdn/cloudfront-function.ts`

**目的**: カスタムCloudFront Functionの作成

#### 機能
- JavaScript関数の作成
- エッジでの処理実行
- リクエスト/レスポンスの改変

#### 使用例
```typescript
import { CloudFrontFunctionConstruct } from '../constructs/cdn/cloudfront-function';

const customFunction = new CloudFrontFunctionConstruct(this, 'CustomFunction', {
  functionCode: `
    function handler(event) {
      // カスタム処理
      return event.request;
    }
  `,
  comment: 'Custom processing function',
});
```

#### プロパティ
- `functionCode`: JavaScript関数のコード
- `comment`: 関数の説明

### BucketPolicyCustomResourceConstruct

**ファイル**: `lib/constructs/cdn/bucket-policy-custom-resource.ts`

**目的**: S3バケットポリシーの動的設定

#### 機能
- CloudFormationカスタムリソースの作成
- S3バケットポリシーの動的更新
- CloudFrontからのアクセス許可設定

#### 使用例
```typescript
import { BucketPolicyCustomResourceConstruct } from '../constructs/cdn/bucket-policy-custom-resource';

const bucketPolicy = new BucketPolicyCustomResourceConstruct(this, 'BucketPolicy', {
  bucket: bucket,
  distribution: distribution,
});
```

#### プロパティ
- `bucket`: 対象のS3バケット
- `distribution`: CloudFrontディストリビューション

## Shared 構成要素

### HostZoneConstruct

**ファイル**: `lib/constructs/shared/host-zone.ts`

**目的**: Route53ホストゾーンの参照

#### 機能
- 既存のRoute53ホストゾーンの参照
- ドメイン名による検索
- 複数スタック間での共有

#### 使用例
```typescript
import { HostZoneConstruct } from '../constructs/shared/host-zone';

const hostZone = new HostZoneConstruct(this, 'HostZone', {
  zoneName: 'nave-wata.net',
});
```

#### プロパティ
- `zoneName`: 検索するホストゾーンのドメイン名

#### 出力
- `hostedZone`: 参照されたRoute53ホストゾーン

### BucketReferenceConstruct

**ファイル**: `lib/constructs/shared/bucket-reference.ts`

**目的**: S3バケットの参照

#### 機能
- 既存のS3バケットの参照
- バケット名による検索
- クロススタック参照

#### 使用例
```typescript
import { BucketReferenceConstruct } from '../constructs/shared/bucket-reference';

const bucketRef = new BucketReferenceConstruct(this, 'BucketRef', {
  bucketName: 'existing-bucket-name',
});
```

#### プロパティ
- `bucketName`: 参照するバケット名

#### 出力
- `bucket`: 参照されたS3バケット

## 型定義 (Interfaces)

### 共通型定義

**ファイル**: `lib/interfaces/shared.ts`

```typescript
export interface BaseProps {
  readonly domainName: string;
}

export interface EnvironmentConfig {
  readonly account: string;
  readonly region: string;
}
```

### ドメイン関連型定義

**ファイル**: `lib/interfaces/domain.ts`

```typescript
export interface CertificateProps extends BaseProps {
  readonly hostedZone: IHostedZone;
}

export interface DnsProps {
  readonly hostedZone: IHostedZone;
  readonly recordName: string;
  readonly target: IDistribution;
}
```

### ストレージ関連型定義

**ファイル**: `lib/interfaces/storage.ts`

```typescript
export interface StaticSiteProps extends BaseProps {
  readonly bucketName?: string;
}
```

### CDN関連型定義

**ファイル**: `lib/interfaces/cdn.ts`

```typescript
export interface CloudFrontProps extends BaseProps {
  readonly certificate: ICertificate;
  readonly bucket: IBucket;
  readonly securityHeadersFunction?: IFunction;
}

export interface SecurityHeadersProps {
  readonly additionalHeaders?: Record<string, string>;
}
```

## 構成要素の使用パターン

### 1. 基本的な使用パターン

```typescript
// 1. 共有リソースの参照
const hostZone = new HostZoneConstruct(this, 'HostZone', {
  zoneName: 'nave-wata.net',
});

// 2. ドメイン関連の設定
const certificate = new CertificateConstruct(this, 'Certificate', {
  domainName: 'tools.nave-wata.net',
  hostedZone: hostZone.hostedZone,
});

// 3. ストレージの設定
const staticSite = new StaticSiteConstruct(this, 'StaticSite', {
  domainName: 'tools.nave-wata.net',
});

// 4. CDNの設定
const securityHeaders = new SecurityHeadersConstruct(this, 'SecurityHeaders');

const cloudfront = new CloudFrontConstruct(this, 'CloudFront', {
  domainName: 'tools.nave-wata.net',
  certificate: certificate.certificate,
  bucket: staticSite.bucket,
  securityHeadersFunction: securityHeaders.function,
});

// 5. DNSの設定
const dns = new DnsConstruct(this, 'Dns', {
  hostedZone: hostZone.hostedZone,
  recordName: 'tools',
  target: cloudfront.distribution,
});
```

### 2. 条件付き構成要素の使用

```typescript
// 環境に応じた構成要素の作成
if (props.environment === 'production') {
  new CloudWatchDashboard(this, 'ProductionDashboard', {
    // 本番環境用の監視ダッシュボード
  });
}

// 機能フラグによる構成要素の制御
if (props.enableLogging) {
  new CloudWatchLogGroup(this, 'AccessLogs', {
    // アクセスログの設定
  });
}
```

### 3. 構成要素の拡張

```typescript
// 既存の構成要素を拡張
export class EnhancedCloudFrontConstruct extends CloudFrontConstruct {
  constructor(scope: Construct, id: string, props: EnhancedCloudFrontProps) {
    super(scope, id, props);
    
    // 追加機能の実装
    if (props.enableWaf) {
      new WebAcl(this, 'WebAcl', {
        // WAF設定
      });
    }
  }
}
```

## ベストプラクティス

### 1. 構成要素の設計原則

- **単一責任**: 各構成要素は単一の責任を持つ
- **再利用性**: 複数のスタックで再利用可能
- **設定可能性**: プロパティによる柔軟な設定
- **型安全性**: TypeScriptの型システムを活用

### 2. エラーハンドリング

```typescript
export class RobustConstruct extends Construct {
  constructor(scope: Construct, id: string, props: RobustConstructProps) {
    super(scope, id);
    
    // 入力値の検証
    if (!props.requiredProperty) {
      throw new Error('requiredProperty is required');
    }
    
    // 条件付きリソース作成
    try {
      this.createResources(props);
    } catch (error) {
      console.error(`Failed to create resources: ${error}`);
      throw error;
    }
  }
}
```

### 3. テスト可能な設計

```typescript
export class TestableConstruct extends Construct {
  public readonly testableProperty: string;
  
  constructor(scope: Construct, id: string, props: TestableConstructProps) {
    super(scope, id);
    
    // テスト可能なプロパティの公開
    this.testableProperty = this.createTestableResource(props);
  }
  
  private createTestableResource(props: TestableConstructProps): string {
    // テスト可能な実装
    return 'testable-value';
  }
}
```

## トラブルシューティング

### 1. 一般的な問題

#### 構成要素の依存関係エラー
```bash
# 依存関係の確認
npx cdk synth | grep -A 5 -B 5 "DependsOn"
```

#### 型定義エラー
```bash
# 型チェック
npx tsc --noEmit
```

### 2. デバッグ方法

```typescript
// デバッグ用のログ出力
export class DebuggableConstruct extends Construct {
  constructor(scope: Construct, id: string, props: DebuggableConstructProps) {
    super(scope, id);
    
    // デバッグ情報の出力
    console.log(`Creating ${id} with props:`, JSON.stringify(props, null, 2));
  }
}
```

詳細な実装については、[architecture.md](./architecture.md) と [development-guide.md](./development-guide.md) を参照してください。
