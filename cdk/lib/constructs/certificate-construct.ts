import { Construct } from 'constructs';
import * as route53 from 'aws-cdk-lib/aws-route53';
import * as acm from 'aws-cdk-lib/aws-certificatemanager';

/**
 * ACM証明書を作成するコンストラクト
 */
export class CertificateConstruct extends Construct {
  public readonly certificate: acm.ICertificate;

  constructor(scope: Construct, id: string, props: {
    domainName: string;
    hostedZone: route53.IHostedZone;
  }) {
    super(scope, id);

    // ACM証明書の作成
    this.certificate = new acm.Certificate(this, 'Certificate', {
      domainName: props.domainName,
      validation: acm.CertificateValidation.fromDns(props.hostedZone),
      keyAlgorithm: acm.KeyAlgorithm.RSA_2048,
    });
  }
}
