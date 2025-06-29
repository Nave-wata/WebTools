import { Construct } from 'constructs';
import * as cloudfront from 'aws-cdk-lib/aws-cloudfront';
import * as origins from 'aws-cdk-lib/aws-cloudfront-origins';
import { BucketPolicyCustomResourceConstruct } from './bucket-policy-custom-resource';
import { BucketReferenceConstruct } from '../shared/bucket-reference';
import { CloudFrontConstructProps } from "../../interfaces/cdn";

/**
 * CloudFrontディストリビューションを作成するコンストラクト
 * 静的サイトホスティング用のCloudFrontディストリビューションを設定し、
 * S3オリジン、SSL証明書、セキュリティヘッダー、エラーページの設定を行う
 */
export class DistributionConstruct extends Construct {
  /** 作成されたCloudFrontディストリビューション */
  public readonly distribution: cloudfront.Distribution;

  /**
   * DistributionConstructのコンストラクタ
   * @param scope - 親となるConstruct
   * @param id - このコンストラクトの一意識別子
   * @param props - CloudFrontディストリビューションの設定プロパティ
   */
  constructor(scope: Construct, id: string, props: CloudFrontConstructProps) {
    super(scope, id);

    // バケット参照の作成（循環参照を避けるため）
    const bucketReference = new BucketReferenceConstruct(this, 'BucketReference', {
      bucketName: props.bucketName,
    });

    // CloudFrontディストリビューションの作成
    this.distribution = new cloudfront.Distribution(this, 'Distribution', {
      defaultRootObject: 'index.html',
      domainNames: [props.domainName],
      certificate: props.certificate,
      defaultBehavior: {
        origin: origins.S3BucketOrigin.withOriginAccessControl(bucketReference.bucket),
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

    // カスタムリソースを使用してバケットポリシーを設定
    new BucketPolicyCustomResourceConstruct(this, 'BucketPolicyCustomResource', {
      bucket: bucketReference.bucket,
      distribution: this.distribution,
    });
  }
}
