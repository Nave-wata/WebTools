import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as acm from 'aws-cdk-lib/aws-certificatemanager';
import { DomainStackProps } from '../interfaces/domain';
import { HostZoneConstruct } from '../constructs/shared/host-zone';
import { CertificateConstruct } from '../constructs/domain/certificate';

/**
 * ドメイン関連のリソースを管理するスタック
 * Route53ホストゾーンの参照とACM証明書の作成を行い、
 * 他のスタックで使用するSSL証明書を提供する
 */
export class DomainStack extends cdk.Stack {
  /** 作成されたACM証明書（他のスタックで使用） */
  public readonly certificate: acm.ICertificate;

  /**
   * DomainStackのコンストラクタ
   * @param scope - 親となるConstruct
   * @param id - このスタックの一意識別子
   * @param props - ドメインスタックの設定プロパティ
   */
  constructor(scope: Construct, id: string, props: cdk.StackProps & DomainStackProps) {
    super(scope, id, props);

    // 参照するデータを取得
    const resourceReferenceConstruct = new HostZoneConstruct(this, 'ResourceReference', {
      zoneName: props.zoneName,
    });

    // ACM証明書の作成
    const certificateConstruct = new CertificateConstruct(this, 'Certificate', {
      domainName: props.domainName,
      hostedZone: resourceReferenceConstruct.hostedZone,
    });
    this.certificate = certificateConstruct.certificate;
  }
}
