import { Construct } from 'constructs';
import * as cloudfront from 'aws-cdk-lib/aws-cloudfront';
import * as origins from 'aws-cdk-lib/aws-cloudfront-origins';
import * as s3 from 'aws-cdk-lib/aws-s3';
import * as acm from 'aws-cdk-lib/aws-certificatemanager';
import { CloudfrontFunction } from "./cloudfront-function";

/**
 * CloudFrontディストリビューションを作成するコンストラクト
 */
export class DistributionConstruct extends Construct {
  public readonly distribution: cloudfront.Distribution;

  constructor(scope: Construct, id: string, props: {
    domainName: string;
    certificate: acm.ICertificate;
    bucket: s3.Bucket;
    responseHeadersPolicy: cloudfront.ResponseHeadersPolicy;
    cloudFrontFunction: CloudfrontFunction,
  }) {
    super(scope, id);

    // CloudFrontディストリビューションの作成
    this.distribution = new cloudfront.Distribution(this, 'Distribution', {
      defaultRootObject: 'index.html',
      domainNames: [props.domainName],
      certificate: props.certificate,
      defaultBehavior: {
        origin: origins.S3BucketOrigin.withOriginAccessControl(props.bucket),
        viewerProtocolPolicy: cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
        responseHeadersPolicy: props.responseHeadersPolicy,
        functionAssociations: [
          {
            function: props.cloudFrontFunction.viewerRequest,
            eventType: cloudfront.FunctionEventType.VIEWER_REQUEST,
          },
        ],
      },
      errorResponses: [
        {
          httpStatus: 403,
          responseHttpStatus: 404,
          responsePagePath: '/404/index.html',
        },
      ],
    });
  }
}
