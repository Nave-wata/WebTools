import { Construct } from 'constructs';
import * as s3 from 'aws-cdk-lib/aws-s3';
import { BucketReferenceConstructProps } from "../../interfaces/shared";

/**
 * S3バケットを参照するためのコンストラクト
 * 循環参照を避けるために使用し、既存のS3バケットを名前で参照して
 * 他のリソースから安全にアクセスできるようにする
 */
export class BucketReferenceConstruct extends Construct {
  /** 参照されたS3バケット */
  public readonly bucket: s3.IBucket;

  /**
   * BucketReferenceConstructのコンストラクタ
   * @param scope - 親となるConstruct
   * @param id - このコンストラクトの一意識別子
   * @param props - バケット参照の設定プロパティ
   */
  constructor(scope: Construct, id: string, props: BucketReferenceConstructProps) {
    super(scope, id);

    // 既存のバケットを参照
    this.bucket = s3.Bucket.fromBucketName(this, 'ReferencedBucket', props.bucketName);
  }
}
