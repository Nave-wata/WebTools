import * as acm from 'aws-cdk-lib/aws-certificatemanager';
import * as cloudfront from 'aws-cdk-lib/aws-cloudfront';
import * as route53 from 'aws-cdk-lib/aws-route53';
import * as s3 from 'aws-cdk-lib/aws-s3';
import { CloudFrontFunction } from "../constructs/cdn/cloudfront-function";

/**
 * CDNスタックのプロパティを定義するインターフェース
 * CloudFrontディストリビューション、DNSレコード、セキュリティヘッダーなどのCDN関連リソースを作成するために必要な設定を含む
 */
export interface CdnStackProps {
  /** CloudFrontディストリビューションで使用するドメイン名 */
  domainName: string;
  /** Route53ホストゾーンの名前 */
  zoneName: string;
  /** DNSレコード名（サブドメイン部分） */
  recordName: string;
  /** CloudFrontで使用するSSL証明書 */
  certificate: acm.ICertificate;
  /** オリジンとして使用するS3バケット */
  bucket: s3.IBucket;
  /** AWS環境設定 */
  env: {
    /** AWSアカウントID */
    account: string;
    /** AWSリージョン */
    region: string;
  };
}

/**
 * DNSレコードコンストラクトのプロパティを定義するインターフェース
 * CloudFrontディストリビューションにRoute53のAliasレコードを作成するために必要な設定を含む
 */
export interface DnsRecordConstructProps {
  /** DNSレコードを作成するRoute53ホストゾーン */
  hostedZone: route53.IHostedZone;
  /** Aliasレコードの対象となるCloudFrontディストリビューション */
  distribution: cloudfront.IDistribution;
  /** 作成するDNSレコード名 */
  recordName: string;
}

/**
 * CloudFrontコンストラクトのプロパティを定義するインターフェース
 * CloudFrontディストリビューションの作成に必要な設定を含む
 */
export interface CloudFrontConstructProps {
  /** ディストリビューションで使用するドメイン名 */
  domainName: string;
  /** HTTPS通信で使用するSSL証明書 */
  certificate: acm.ICertificate;
  /** オリジンとして使用するS3バケット名 */
  bucketName: string;
  /** レスポンスヘッダーポリシー（セキュリティヘッダーの設定） */
  responseHeadersPolicy: cloudfront.ResponseHeadersPolicy;
  /** ビューワーリクエスト時に実行するCloudFront Function */
  cloudFrontFunction: CloudFrontFunction,
}

/**
 * CloudFront Functionコンストラクトのプロパティを定義するインターフェース
 * エッジでのリクエスト処理を行うCloudFront Functionの作成に必要な設定を含む
 */
export interface CloudFrontFunctionConstructProps {
  /** Function名の生成に使用するドメイン名 */
  domainName: string;
}

/**
 * セキュリティヘッダーコンストラクトのプロパティを定義するインターフェース
 * CloudFrontのレスポンスヘッダーポリシーでセキュリティヘッダーを設定するために必要な設定を含む
 */
export interface SecurityHeadersConstructProps {
  /** レスポンスヘッダーポリシーの名前 */
  policyName: string;
}

/**
 * バケットポリシーカスタムリソースコンストラクトのプロパティを定義するインターフェース
 * CloudFrontからS3バケットへのアクセスを許可するバケットポリシーを設定するために必要な設定を含む
 */
export interface BucketPolicyCustomResourceConstructProps {
  /** ポリシーを設定するS3バケット */
  bucket: s3.IBucket;
  /** アクセスを許可するCloudFrontディストリビューション */
  distribution: cloudfront.Distribution;
}
