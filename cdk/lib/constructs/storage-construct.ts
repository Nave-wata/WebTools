import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as s3 from 'aws-cdk-lib/aws-s3';
import * as iam from 'aws-cdk-lib/aws-iam';
import * as cloudfront from 'aws-cdk-lib/aws-cloudfront';

/**
 * S3ストレージを作成するコンストラクト
 */
export class StorageConstruct extends Construct {
  public readonly bucket: s3.Bucket;

  constructor(scope: Construct, id: string, props: {
    bucketName: string;
  }) {
    super(scope, id);

    // S3バケットの作成
    this.bucket = new s3.Bucket(this, 'WebsiteBucket', {
      bucketName: this.getBucketName(props.bucketName),
      blockPublicAccess: s3.BlockPublicAccess.BLOCK_ALL,
      versioned: false,
      encryption: s3.BucketEncryption.S3_MANAGED,
      enforceSSL: true,
      removalPolicy: cdk.RemovalPolicy.RETAIN,
    });
  }

  /**
   * CloudFrontからのアクセス権限を付与
   */
  public allowCloudFrontAccess(distribution: cloudfront.Distribution): void {
    this.bucket.addToResourcePolicy(
      new iam.PolicyStatement({
        actions: ['s3:GetObject'],
        resources: [this.bucket.arnForObjects('*')],
        principals: [new iam.ServicePrincipal('cloudfront.amazonaws.com')],
        conditions: {
          StringEquals: {
            'AWS:SourceArn': `arn:aws:cloudfront::${cdk.Stack.of(this).account}:distribution/${distribution.distributionId}`,
          },
        },
      })
    );
  }

  /**
   * バケット名を取得
   *
   * @param baseName
   * @private
   */
  private getBucketName(baseName: string): string {
    const prefix = baseName.replace(/\./g, '-');
    const randomId = crypto
      .randomUUID()
      .toString()
      .replace(/-/g, '')
      .substring(0, 16);

    return `${prefix}-${randomId}`;
  }
}
