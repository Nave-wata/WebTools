/**
 * バケット参照コンストラクトのプロパティを定義するインターフェース
 * 既存のS3バケットを参照するために必要な設定を含む
 */
export interface BucketReferenceConstructProps {
  /** 参照するS3バケットの名前 */
  bucketName: string;
}

/**
 * ホストゾーンコンストラクトのプロパティを定義するインターフェース
 * Route53ホストゾーンを参照するために必要な設定を含む
 */
export interface HostZoneConstructProps {
  /** 参照するRoute53ホストゾーンの名前 */
  zoneName: string;
}
