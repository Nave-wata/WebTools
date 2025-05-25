import * as cloudfront from 'aws-cdk-lib/aws-cloudfront';
import { Construct } from 'constructs';

/**
 * CloudFront Function ベースコンストラクタ
 */
export class CloudfrontFunction extends Construct {
  public readonly viewerRequest: cloudfront.Function;

  constructor(scope: Construct, id: string, props: {
    domainName: string;
  }) {
    super(scope, id);

    this.viewerRequest = new cloudfront.Function(this, 'Function', {
      code: cloudfront.FunctionCode.fromFile({
        filePath: 'lib/functions/cloudfront/viewer-request.js',
      }),
      functionName: `${props.domainName.replace(/\./g, "-")}_viewer-request`,
      comment: 'Denies access to the default CloudFront distribution domain. And appends index.html to URIs that end with / or have no file extension.',
      runtime: cloudfront.FunctionRuntime.JS_2_0,
    });
  }
}
