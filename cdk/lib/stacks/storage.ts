import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as s3 from 'aws-cdk-lib/aws-s3';
import { StorageStackProps } from '../interfaces/storage';
import { StorageConstruct } from '../constructs/storage/static-site';

/**
 * ストレージ関連のリソースを管理するスタック
 * 静的サイトホスティング用のS3バケットを作成し、
 * セキュリティ設定を適用して他のスタックで使用できるようにする
 */
export class StorageStack extends cdk.Stack {
  /** 作成されたS3バケット（他のスタックで使用） */
  public readonly bucket: s3.Bucket;
  /** ストレージコンストラクトのインスタンス */
  private readonly storage: StorageConstruct;

  /**
   * StorageStackのコンストラクタ
   * @param scope - 親となるConstruct
   * @param id - このスタックの一意識別子
   * @param props - ストレージスタックの設定プロパティ
   */
  constructor(scope: Construct, id: string, props: cdk.StackProps & StorageStackProps) {
    super(scope, id, props);

    // S3ストレージの作成
    this.storage = new StorageConstruct(this, 'Storage', {
      bucketName: props.domainName,
    });
    this.bucket = this.storage.bucket;
  }
}
