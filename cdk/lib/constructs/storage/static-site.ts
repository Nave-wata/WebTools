import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as s3 from 'aws-cdk-lib/aws-s3';
import { StaticSiteConstructProps } from "../../interfaces/storage";

/**
 * S3ストレージを作成するコンストラクト
 * 静的サイトホスティング用のS3バケットを作成し、
 * セキュリティ設定（パブリックアクセスブロック、SSL強制、暗号化）を適用する
 */
export class StorageConstruct extends Construct {
  /** 作成されたS3バケット */
  public readonly bucket: s3.Bucket;

  /**
   * StorageConstructのコンストラクタ
   * @param scope - 親となるConstruct
   * @param id - このコンストラクトの一意識別子
   * @param props - S3バケットの設定プロパティ
   */
  constructor(scope: Construct, id: string, props: StaticSiteConstructProps) {
    super(scope, id);

    // S3バケットの作成
    this.bucket = new s3.Bucket(this, 'WebsiteBucket', {
      bucketName: props.bucketName.replace(/\./g, '-'),
      blockPublicAccess: s3.BlockPublicAccess.BLOCK_ALL,
      versioned: false,
      encryption: s3.BucketEncryption.S3_MANAGED,
      enforceSSL: true,
      removalPolicy: cdk.RemovalPolicy.RETAIN,
    });
  }

}
