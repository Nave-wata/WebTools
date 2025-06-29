import { Construct } from 'constructs';
import * as route53 from 'aws-cdk-lib/aws-route53';
import { HostZoneConstructProps } from '../../interfaces/shared';

/**
 * 既存のホストゾーンを参照するためのコンストラクト
 * Route53で既存のホストゾーンを検索し、
 * 他のリソースから参照できるようにする
 */
export class HostZoneConstruct extends Construct {
  /** 参照されたRoute53ホストゾーン */
  public readonly hostedZone: route53.IHostedZone;

  /**
   * HostZoneConstructのコンストラクタ
   * @param scope - 親となるConstruct
   * @param id - このコンストラクトの一意識別子
   * @param props - ホストゾーン参照の設定プロパティ
   */
  constructor(scope: Construct, id: string, props: HostZoneConstructProps) {
    super(scope, id);

    // 既存のホストゾーンを検索
    this.hostedZone = route53.HostedZone.fromLookup(this, 'HostedZone', {
      domainName: props.zoneName,
    });
  }
}
