import { Construct } from 'constructs';
import * as route53 from 'aws-cdk-lib/aws-route53';
import * as cloudfront from 'aws-cdk-lib/aws-cloudfront';
import * as targets from 'aws-cdk-lib/aws-route53-targets';

/**
 * DNSレコードを作成するコンストラクト
 */
export class DnsRecordConstruct extends Construct {
  constructor(scope: Construct, id: string, props: {
    hostedZone: route53.IHostedZone;
    distribution: cloudfront.Distribution;
    recordName: string;
  }) {
    super(scope, id);

    // Route53 DNSレコードの作成
    new route53.ARecord(this, 'AliasRecord', {
      zone: props.hostedZone,
      recordName: props.recordName,
      target: route53.RecordTarget.fromAlias(new targets.CloudFrontTarget(props.distribution)),
    });
  }
}
