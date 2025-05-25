import * as cdk from 'aws-cdk-lib';
import { Template } from 'aws-cdk-lib/assertions';
import * as cloudfront from 'aws-cdk-lib/aws-cloudfront';
import * as route53 from 'aws-cdk-lib/aws-route53';
import * as origins from 'aws-cdk-lib/aws-cloudfront-origins';
import * as s3 from 'aws-cdk-lib/aws-s3';
import { DnsStack } from '../../lib/stacks';
import { DnsRecordConstruct, ResourcesReferenceConstruct } from '../../lib/constructs';

// モックのリソースリファレンスコンストラクトを作成
jest.mock('../../lib/constructs/resources-reference-construct', () => {
  return {
    ResourcesReferenceConstruct: jest.fn().mockImplementation((scope) => {
      const mockHostedZone = new route53.HostedZone(scope, 'MockHostedZone', {
        zoneName: 'example.com'
      });
      return {
        hostedZone: mockHostedZone
      };
    })
  };
});

// モックのDNSレコードコンストラクトを作成
jest.mock('../../lib/constructs/dns-record-construct', () => {
  return {
    DnsRecordConstruct: jest.fn().mockImplementation(() => {
      return {};
    })
  };
});

describe('DnsStack', () => {
  let app: cdk.App;
  let stack: DnsStack;
  let template: Template;

  beforeEach(() => {
    jest.clearAllMocks();
    app = new cdk.App();

    // 単一スタック内に全てのリソースを作成
    const testStack = new cdk.Stack(app, 'TestStack', {
      env: { account: '123456789012', region: 'us-east-1' }
    });
    
    // テスト用のバケットを作成
    const bucket = new s3.Bucket(testStack, 'TestBucket');
    
    // テスト用の実際のCloudFrontディストリビューションを作成
    const distribution = new cloudfront.Distribution(testStack, 'TestDistribution', {
      defaultBehavior: {
        origin: origins.S3BucketOrigin.withOriginAccessControl(bucket)
      }
    });

    // Create the stack with test properties
    stack = new DnsStack(app, 'TestDnsStack', {
      env: { account: '123456789012', region: 'us-east-1' },
      zoneName: 'example.com',
      distribution: distribution,
      recordName: 'test',
    });

    // Generate CloudFormation template
    template = Template.fromStack(stack);
  });

  test('ResourcesReferenceConstruct is called with correct parameters', () => {
    expect(ResourcesReferenceConstruct).toHaveBeenCalledWith(
      expect.anything(),
      'ResourceReference',
      expect.objectContaining({
        zoneName: 'example.com'
      })
    );
  });

  test('DnsRecordConstruct is called with correct parameters', () => {
    expect(DnsRecordConstruct).toHaveBeenCalledWith(
      expect.anything(),
      'DnsRecord',
      expect.objectContaining({
        recordName: 'test'
      })
    );
  });

  test('Stack Outputs Created', () => {
    // Verify that the stack has the expected outputs
    template.hasOutput('WebsiteURL', {
      Value: 'https://test.example.com'
    });
  });
});
