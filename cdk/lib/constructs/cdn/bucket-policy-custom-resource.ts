import { Construct } from 'constructs';
import * as cdk from 'aws-cdk-lib';
import * as cr from 'aws-cdk-lib/custom-resources';
import { BucketPolicyCustomResourceConstructProps } from "../../interfaces/cdn";

/**
 * S3バケットポリシーを設定するカスタムリソース
 * CloudFrontディストリビューション作成後に確実にバケットポリシーを設定し、
 * CloudFrontサービスプリンシパルからのアクセスのみを許可するセキュアなポリシーを適用する
 */
export class BucketPolicyCustomResourceConstruct extends Construct {
  /**
   * BucketPolicyCustomResourceConstructのコンストラクタ
   * @param scope - 親となるConstruct
   * @param id - このコンストラクトの一意識別子
   * @param props - バケットポリシー設定のプロパティ
   */
  constructor(scope: Construct, id: string, props: BucketPolicyCustomResourceConstructProps) {
    super(scope, id);

    // バケットポリシードキュメントを作成
    // CloudFrontサービスプリンシパルからのアクセスのみを許可
    const policyDocument = {
      Version: '2012-10-17',
      Statement: [
        {
          // ポリシーステートメントの識別子
          Sid: 'AllowCloudFrontServicePrincipal',

          // アクセス許可
          Effect: 'Allow',

          // CloudFrontサービスプリンシパルを指定
          // これによりCloudFrontサービスからのアクセスを許可
          Principal: {
            Service: 'cloudfront.amazonaws.com'
          },

          // 許可するアクション（オブジェクトの取得のみ）
          Action: 's3:GetObject',

          // 対象リソース（バケット内の全オブジェクト）
          Resource: `${props.bucket.bucketArn}/*`,

          // 条件：特定のCloudFrontディストリビューションからのアクセスのみを許可
          // AWS:SourceArnを使用してディストリビューションを特定
          Condition: {
            StringEquals: {
              'AWS:SourceArn': `arn:aws:cloudfront::${cdk.Stack.of(this).account}:distribution/${props.distribution.distributionId}`
            }
          }
        }
      ]
    };

    // カスタムリソースを作成してバケットポリシーを設定
    // CloudFormationのライフサイクル（作成、更新、削除）に対応
    new cr.AwsCustomResource(this, 'BucketPolicyCustomResource', {
      // リソース作成時の処理
      // S3のputBucketPolicy APIを呼び出してポリシーを設定
      onCreate: {
        service: 'S3',
        action: 'putBucketPolicy',
        parameters: {
          Bucket: props.bucket.bucketName,
          Policy: JSON.stringify(policyDocument)
        },
        // 物理リソースIDを設定（CloudFormationが変更を検出するため）
        physicalResourceId: cr.PhysicalResourceId.of(`bucket-policy-${props.bucket.bucketName}`)
      },

      // リソース更新時の処理
      // ポリシーの内容が変更された場合に再設定
      onUpdate: {
        service: 'S3',
        action: 'putBucketPolicy',
        parameters: {
          Bucket: props.bucket.bucketName,
          Policy: JSON.stringify(policyDocument)
        },
        physicalResourceId: cr.PhysicalResourceId.of(`bucket-policy-${props.bucket.bucketName}`)
      },

      // リソース削除時の処理
      // スタック削除時にバケットポリシーを削除
      onDelete: {
        service: 'S3',
        action: 'deleteBucketPolicy',
        parameters: {
          Bucket: props.bucket.bucketName
        }
      },

      // カスタムリソースが必要とするIAM権限を設定
      // S3バケットとそのオブジェクトに対する操作権限を付与
      policy: cr.AwsCustomResourcePolicy.fromSdkCalls({
        resources: [
          props.bucket.bucketArn,
          `${props.bucket.bucketArn}/*`
        ]
      })
    });
  }
}
