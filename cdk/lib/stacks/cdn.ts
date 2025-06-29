import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as cloudfront from 'aws-cdk-lib/aws-cloudfront';
import { CdnStackProps } from '../interfaces/cdn';
import { SecurityHeadersConstruct } from '../constructs/cdn/security-headers-construct';
import { CloudFrontFunction } from '../constructs/cdn/cloudfront-function';
import { DistributionConstruct } from '../constructs/cdn/cloudfront';
import { HostZoneConstruct } from '../constructs/shared/host-zone';
import { DnsRecordConstruct } from '../constructs/domain/dns';

/**
 * CDN関連のリソースを管理するスタック
 * CloudFrontディストリビューション、セキュリティヘッダーポリシー、
 * CloudFront Function、DNSレコードを統合して静的サイトのCDNを構築する
 */
export class CdnStack extends cdk.Stack {
  /** 作成されたCloudFrontディストリビューション */
  public readonly distribution: cloudfront.Distribution;

  /**
   * CdnStackのコンストラクタ
   * @param scope - 親となるConstruct
   * @param id - このスタックの一意識別子
   * @param props - CDNスタックの設定プロパティ
   */
  constructor(scope: Construct, id: string, props: cdk.StackProps & CdnStackProps) {
    super(scope, id, props);

    // セキュリティヘッダーポリシーの作成
    const securityHeaders = new SecurityHeadersConstruct(this, 'SecurityHeaders', {
      policyName: props.domainName,
    });

    // CloudFront Function の作成
    const cloudFrontFunction = new CloudFrontFunction(this, 'CloudFrontFunction', {
      domainName: props.domainName,
    });

    // CloudFrontディストリビューションの作成
    const distribution = new DistributionConstruct(this, 'Distribution', {
      domainName: props.domainName,
      certificate: props.certificate,
      bucketName: props.bucket.bucketName,
      responseHeadersPolicy: securityHeaders.responseHeadersPolicy,
      cloudFrontFunction: cloudFrontFunction,
    });
    this.distribution = distribution.distribution;

    // DNSレコードの作成
    const resourceReferenceConstruct = new HostZoneConstruct(this, 'DnsResourceReference', {
      zoneName: props.zoneName,
    });

    new DnsRecordConstruct(this, 'DnsRecord', {
      hostedZone: resourceReferenceConstruct.hostedZone,
      distribution: this.distribution,
      recordName: props.recordName,
    });
  }
}
