import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as cloudfront from 'aws-cdk-lib/aws-cloudfront';
import { SecurityHeadersConstructProps } from "../../interfaces/cdn";

/**
 * セキュリティヘッダーポリシーを作成するコンストラクト
 * CloudFrontのレスポンスヘッダーポリシーを設定し、
 * CSP、HSTS、XSS保護、フレーム保護などのセキュリティヘッダーを自動付与する
 */
export class SecurityHeadersConstruct extends Construct {
  /** 作成されたレスポンスヘッダーポリシー */
  public readonly responseHeadersPolicy: cloudfront.ResponseHeadersPolicy;

  /**
   * SecurityHeadersConstructのコンストラクタ
   * @param scope - 親となるConstruct
   * @param id - このコンストラクトの一意識別子
   * @param props - セキュリティヘッダーポリシーの設定プロパティ
   */
  constructor(scope: Construct, id: string, props: SecurityHeadersConstructProps) {
    super(scope, id);

    // セキュリティヘッダーポリシーの作成
    this.responseHeadersPolicy = new cloudfront.ResponseHeadersPolicy(this, 'SecurityHeadersPolicy', {
      responseHeadersPolicyName: `${props.policyName.replace(/\./g, '-')}_security-headers`,
      securityHeadersBehavior: {
        contentSecurityPolicy: {
          contentSecurityPolicy: "default-src 'self'; img-src 'self' data:; script-src 'self' 'unsafe-inline' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; connect-src 'self';",
          override: true,
        },
        strictTransportSecurity: {
          accessControlMaxAge: cdk.Duration.days(2 * 365),
          includeSubdomains: true,
          preload: true,
          override: true,
        },
        contentTypeOptions: {
          override: true,
        },
        frameOptions: {
          frameOption: cloudfront.HeadersFrameOption.DENY,
          override: true,
        },
        xssProtection: {
          protection: true,
          modeBlock: true,
          override: true,
        },
        referrerPolicy: {
          referrerPolicy: cloudfront.HeadersReferrerPolicy.STRICT_ORIGIN_WHEN_CROSS_ORIGIN,
          override: true,
        },
      },
    });
  }
}
