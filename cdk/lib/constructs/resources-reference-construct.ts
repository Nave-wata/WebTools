import { Construct } from 'constructs';
import * as route53 from 'aws-cdk-lib/aws-route53';

/**
 * 既存のAWSリソースを参照するためのコンストラクト
 */
export class ResourcesReferenceConstruct extends Construct {
  public readonly hostedZone: route53.IHostedZone;

  constructor(scope: Construct, id: string, props: {
    zoneName: string;
  }) {
    super(scope, id);

    // 既存のホストゾーンを検索
    this.hostedZone = route53.HostedZone.fromLookup(this, 'HostedZone', {
      domainName: props.zoneName,
    });
  }
}
