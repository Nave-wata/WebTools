/**
 * WebTools CDKアプリケーションのエントリーポイント
 * 
 * このファイルは静的サイトホスティングのためのAWSインフラストラクチャを定義し、
 * 以下のスタックを順次デプロイします：
 * 1. DomainStack - Route53ホストゾーンとSSL証明書の管理
 * 2. StorageStack - S3バケットによる静的サイトストレージ
 * 3. CdnStack - CloudFrontディストリビューションとDNSレコード
 */

import * as cdk from 'aws-cdk-lib';
import { requireEnv } from "../utils";
import { DomainStack } from "../lib/stacks/domain";
import { StorageStack } from '../lib/stacks/storage';
import { CdnStack } from "../lib/stacks/cdn";

/** CDKアプリケーションのインスタンス */
const app = new cdk.App();

// ドメイン設定
/** スタック名のプレフィックス */
const stackPrefix = 'ToolsNaveWataNet';
/** 静的サイトのドメイン名 */
const domainName = 'tools.nave-wata.net';
/** Route53ホストゾーンの名前 */
const zoneName = 'nave-wata.net';
/** DNSレコード名（サブドメイン部分） */
const recordName = 'tools';

// ドメインスタックのデプロイ
const domainStack = new DomainStack(app, `${stackPrefix}-DomainStack`, {
  env: {
    account: requireEnv("AWS_DEFAULT_ACCOUNT"),
    region: "us-east-1",
  },
  zoneName: zoneName,
  domainName: domainName,
});

// ストレージスタックのデプロイ
const storageStack = new StorageStack(app, `${stackPrefix}-StorageStack`, {
  env: {
    account: requireEnv("AWS_DEFAULT_ACCOUNT"),
    region: requireEnv("AWS_DEFAULT_REGION"),
  },
  domainName: domainName,
});

// CDNスタックのデプロイ
const cdnStack = new CdnStack(app, `${stackPrefix}-CdnStack`, {
  env: {
    account: requireEnv("AWS_DEFAULT_ACCOUNT"),
    region: requireEnv("AWS_DEFAULT_REGION"),
  },
  domainName: domainName,
  zoneName: zoneName,
  recordName: recordName,

  certificate: domainStack.certificate,
  bucket: storageStack.bucket,

  // クロスリージョンの参照を有効にする
  crossRegionReferences: true,
});

// 依存関係を明示的に指定
cdnStack.addDependency(domainStack);
cdnStack.addDependency(storageStack);
