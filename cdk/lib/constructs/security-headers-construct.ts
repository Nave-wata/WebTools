import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as cloudfront from 'aws-cdk-lib/aws-cloudfront';

/**
 * セキュリティヘッダーポリシーを作成するコンストラクト
 */
export class SecurityHeadersConstruct extends Construct {
  public readonly responseHeadersPolicy: cloudfront.ResponseHeadersPolicy;

  constructor(scope: Construct, id: string, props: {
    policyName: string;
  }) {
    super(scope, id);

    // セキュリティヘッダーポリシーの作成
    this.responseHeadersPolicy = new cloudfront.ResponseHeadersPolicy(this, 'SecurityHeadersPolicy', {
      responseHeadersPolicyName: `${props.policyName.replace(/\./g, '-')}_security-headers`,
      securityHeadersBehavior: {
        contentSecurityPolicy: {
          contentSecurityPolicy: "default-src 'self'; img-src 'self' data:; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; connect-src 'self';",
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
