import * as route53 from 'aws-cdk-lib/aws-route53';

/**
 * ドメインスタックのプロパティを定義するインターフェース
 * ドメイン関連のリソース（Route53ホストゾーン、SSL証明書）を作成するために必要な設定を含む
 */
export interface DomainStackProps {
  /** Route53ホストゾーンの名前（例: "example.com"） */
  zoneName: string;
  /** SSL証明書を発行するドメイン名（例: "www.example.com"） */
  domainName: string;
  /** AWS環境設定 */
  env: {
    /** AWSアカウントID */
    account: string;
    /** AWSリージョン（例: "us-east-1"） */
    region: string;
  };
}

/**
 * SSL証明書コンストラクトのプロパティを定義するインターフェース
 * ACM（AWS Certificate Manager）でSSL証明書を作成するために必要な設定を含む
 */
export interface CertificateConstructProps {
  /** SSL証明書を発行するドメイン名 */
  domainName: string;
  /** DNS検証に使用するRoute53ホストゾーン */
  hostedZone: route53.IHostedZone;
}
