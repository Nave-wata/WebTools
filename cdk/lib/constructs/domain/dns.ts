import { Construct } from 'constructs';
import * as route53 from 'aws-cdk-lib/aws-route53';
import * as targets from 'aws-cdk-lib/aws-route53-targets';
import { DnsRecordConstructProps } from "../../interfaces/cdn";

/**
 * DNSレコードを作成するコンストラクト
 * Route53でCloudFrontディストリビューションへのAliasレコード（Aレコード）を作成し、
 * カスタムドメインでのアクセスを可能にする
 */
export class DnsRecordConstruct extends Construct {
  /**
   * DnsRecordConstructのコンストラクタ
   * @param scope - 親となるConstruct
   * @param id - このコンストラクトの一意識別子
   * @param props - DNSレコードの設定プロパティ
   */
  constructor(scope: Construct, id: string, props: DnsRecordConstructProps) {
    super(scope, id);

    // Route53 DNSレコードの作成
    new route53.ARecord(this, 'AliasRecord', {
      zone: props.hostedZone,
      recordName: props.recordName,
      target: route53.RecordTarget.fromAlias(new targets.CloudFrontTarget(props.distribution)),
    });
  }
}
