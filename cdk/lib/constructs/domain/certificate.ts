import { Construct } from 'constructs';
import * as acm from 'aws-cdk-lib/aws-certificatemanager';
import { CertificateConstructProps } from '../../interfaces/domain';

/**
 * ACM証明書を作成するコンストラクト
 * AWS Certificate Manager（ACM）を使用してSSL/TLS証明書を作成し、
 * Route53でのDNS検証を自動化する
 */
export class CertificateConstruct extends Construct {
  /** 作成されたACM証明書 */
  public readonly certificate: acm.ICertificate;

  /**
   * CertificateConstructのコンストラクタ
   * @param scope - 親となるConstruct
   * @param id - このコンストラクトの一意識別子
   * @param props - SSL証明書の設定プロパティ
   */
  constructor(scope: Construct, id: string, props: CertificateConstructProps) {
    super(scope, id);

    // ACM証明書の作成
    this.certificate = new acm.Certificate(this, 'Certificate', {
      domainName: props.domainName,
      validation: acm.CertificateValidation.fromDns(props.hostedZone),
      keyAlgorithm: acm.KeyAlgorithm.RSA_2048,
    });
  }
}
