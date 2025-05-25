import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as acm from 'aws-cdk-lib/aws-certificatemanager';
import { CertificateConstruct, ResourcesReferenceConstruct } from "../constructs";
import { DefaultStackPropsInterface } from "../../bin/app";

/**
 * 基盤インフラストラクチャスタック
 *
 * 証明書の作成を担当します
 */
export class BaseInfrastructureStack extends cdk.Stack {
  public readonly certificate: acm.ICertificate;

  constructor(scope: Construct, id: string, props: cdk.StackProps & DefaultStackPropsInterface & {
    domainName: string;
  }) {
    super(scope, id, props);

    // 参照するデータを取得
    const resourceReferenceConstruct = new ResourcesReferenceConstruct(this, 'ResourceReference', {
      zoneName: props.zoneName,
    });

    // ACM証明書の作成
    const certificateConstruct = new CertificateConstruct(this, 'Certificate', {
      domainName: props.domainName,
      hostedZone: resourceReferenceConstruct.hostedZone,
    });
    this.certificate = certificateConstruct.certificate;

    // スタック出力
    new cdk.CfnOutput(this, 'CertificateArn', { value: this.certificate.certificateArn });
    new cdk.CfnOutput(this, 'HostedZoneId', { value: resourceReferenceConstruct.hostedZone.hostedZoneId });
  }
}
