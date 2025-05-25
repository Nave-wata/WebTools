import * as cdk from 'aws-cdk-lib';
import { BaseInfrastructureStack, DnsStack, WebsiteStack } from '../lib/stacks';
import { requireEnv } from "../utils";

const app = new cdk.App();

// ドメイン設定
const stackPrefix = 'ToolsNaveWataNetStack';
const domainName = 'tools.nave-wata.net';
const zoneName = 'nave-wata.net';
const recordName = 'tools';

// デフォルト引数のインターフェース
export interface DefaultStackPropsInterface {
  zoneName: string;
  env: {
    account: string;
    region: string;
  }
}

// デフォルト引数
const defaultStackProps: cdk.StackProps & DefaultStackPropsInterface = {
  zoneName: zoneName,
  env: {
    account: requireEnv("CDK_DEFAULT_ACCOUNT"),
    region: requireEnv("CDK_DEFAULT_REGION"),
  },
};

// 基盤インフラストラクチャスタックのデプロイ
const baseStack = new BaseInfrastructureStack(app, `${stackPrefix}-BaseInfraStack`, {
  ...defaultStackProps,
  domainName: domainName,
});

// ウェブサイトスタックのデプロイ
const websiteStack = new WebsiteStack(app, `${stackPrefix}-WebsiteStack`, {
  ...defaultStackProps,
  domainName: domainName,
  certificate: baseStack.certificate,
});

// DNSスタックのデプロイ
const dnsStack = new DnsStack(app, `${stackPrefix}-DnsStack`, {
  ...defaultStackProps,
  distribution: websiteStack.distribution,
  recordName: recordName,
});

// スタック間の依存関係を明示的に設定
websiteStack.addDependency(baseStack);
dnsStack.addDependency(websiteStack);
