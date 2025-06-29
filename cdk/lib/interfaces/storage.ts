/**
 * ストレージスタックのプロパティを定義するインターフェース
 * 静的サイトホスティング用のS3バケットとその関連リソースを作成するために必要な設定を含む
 */
export interface StorageStackProps {
  /** 静的サイトのドメイン名（バケット名の生成に使用） */
  domainName: string;
  /** AWS環境設定 */
  env: {
    /** AWSアカウントID */
    account: string;
    /** AWSリージョン（例: "us-east-1"） */
    region: string;
  };
}

/**
 * 静的サイトコンストラクトのプロパティを定義するインターフェース
 * S3バケットを使用した静的サイトホスティングの設定に必要なプロパティを含む
 */
export interface StaticSiteConstructProps {
  /** 作成するS3バケットの名前 */
  bucketName: string;
}
