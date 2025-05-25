import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as cloudfront from 'aws-cdk-lib/aws-cloudfront';
import { DnsRecordConstruct, ResourcesReferenceConstruct } from '../constructs';
import { DefaultStackPropsInterface } from "../../bin/app";

/**
 * DNSレコードスタック
 *
 * Route53 DNSレコードの作成を担当します
 */
export class DnsStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props: cdk.StackProps & DefaultStackPropsInterface & {
    distribution: cloudfront.IDistribution;
    recordName: string;
  }) {
    super(scope, id, props);

    // 参照するデータを取得
    const resourceReferenceConstruct = new ResourcesReferenceConstruct(this, 'ResourceReference', {
      zoneName: props.zoneName,
    })

    // DNSレコードの作成
    new DnsRecordConstruct(this, 'DnsRecord', {
      hostedZone: resourceReferenceConstruct.hostedZone,
      distribution: props.distribution,
      recordName: props.recordName,
    });

    // スタック出力
    new cdk.CfnOutput(this, 'WebsiteURL', {
      value: `https://${props.recordName}.${resourceReferenceConstruct.hostedZone.zoneName}`
    });
  }
}
