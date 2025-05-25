import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as cloudfront from 'aws-cdk-lib/aws-cloudfront';
import * as s3 from 'aws-cdk-lib/aws-s3';
import * as acm from 'aws-cdk-lib/aws-certificatemanager';
import { CloudFrontFunction, DistributionConstruct, SecurityHeadersConstruct, StorageConstruct, } from "../constructs";
import { DefaultStackPropsInterface } from "../../bin/app";

/**
 * ウェブサイトリソーススタック
 *
 * S3バケット、CloudFrontディストリビューション、セキュリティポリシーなどを担当します
 */
export class WebsiteStack extends cdk.Stack {
  public readonly bucket: s3.Bucket;
  public readonly distribution: cloudfront.Distribution;

  constructor(scope: Construct, id: string, props: cdk.StackProps & DefaultStackPropsInterface & {
    domainName: string;
    certificate: acm.ICertificate;
  }) {
    super(scope, id, props);

    // S3ストレージの作成
    const storage = new StorageConstruct(this, 'Storage', {
      bucketName: props.domainName,
    });
    this.bucket = storage.bucket;

    // セキュリティヘッダーポリシーの作成
    const securityHeaders = new SecurityHeadersConstruct(this, 'SecurityHeaders', {
      policyName: props.domainName,
    });

    // CloudFront Function の作成
    const cloudFrontFunction = new CloudFrontFunction(this, 'CloudFrontFunction', {
      domainName: props.domainName,
    });

    // CloudFrontディストリビューションの作成
    const distribution = new DistributionConstruct(this, 'Distribution', {
      domainName: props.domainName,
      certificate: props.certificate,
      bucket: this.bucket,
      responseHeadersPolicy: securityHeaders.responseHeadersPolicy,
      cloudFrontFunction: cloudFrontFunction,
    });
    this.distribution = distribution.distribution;

    // S3バケットへのCloudFrontアクセス権限を付与
    storage.allowCloudFrontAccess(this.distribution);

    // スタック出力
    new cdk.CfnOutput(this, 'BucketName', { value: this.bucket.bucketName });
    new cdk.CfnOutput(this, 'DistributionId', { value: this.distribution.distributionId });
    new cdk.CfnOutput(this, 'DistributionDomainName', { value: this.distribution.distributionDomainName });
  }
}
