import * as cloudfront from 'aws-cdk-lib/aws-cloudfront';
import { Construct } from 'constructs';
import { CloudFrontFunctionConstructProps } from "../../interfaces/cdn";

/**
 * CloudFront Function ベースコンストラクタ
 * エッジでのリクエスト処理を行うCloudFront Functionを作成し、
 * デフォルトドメインへのアクセス拒否とindex.htmlの自動追加機能を提供する
 */
export class CloudFrontFunction extends Construct {
  /** ビューワーリクエスト時に実行されるCloudFront Function */
  public readonly viewerRequest: cloudfront.Function;

  /**
   * CloudFrontFunctionのコンストラクタ
   * @param scope - 親となるConstruct
   * @param id - このコンストラクトの一意識別子
   * @param props - CloudFront Functionの設定プロパティ
   */
  constructor(scope: Construct, id: string, props: CloudFrontFunctionConstructProps) {
    super(scope, id);

    this.viewerRequest = new cloudfront.Function(this, 'Function', {
      code: cloudfront.FunctionCode.fromFile({
        filePath: 'assets/functions/cloudfront/viewer-request.js',
      }),
      functionName: `${props.domainName.replace(/\./g, "-")}_viewer-request`,
      comment: 'Denies access to the default CloudFront distribution domain. And appends index.html to URIs that end with / or have no file extension.',
      runtime: cloudfront.FunctionRuntime.JS_2_0,
    });
  }
}
