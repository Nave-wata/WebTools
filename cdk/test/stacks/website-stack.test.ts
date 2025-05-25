import * as cdk from 'aws-cdk-lib';
import { Template } from 'aws-cdk-lib/assertions';
import * as acm from 'aws-cdk-lib/aws-certificatemanager';
import { WebsiteStack } from '../../lib/stacks';

describe('WebsiteStack', () => {
  let app: cdk.App;
  let stack: WebsiteStack;
  let template: Template;

  beforeEach(() => {
    app = new cdk.App();

    // Create the stack first
    stack = new WebsiteStack(app, 'TestWebsiteStack', {
      env: { account: '123456789012', region: 'us-east-1' },
      zoneName: 'example.com',
      domainName: 'test.example.com',
      // Use a mock certificate that doesn't require cross-stack references
      certificate: {
        certificateArn: 'arn:aws:acm:us-east-1:123456789012:certificate/mock-certificate',
        applyRemovalPolicy: () => {},
        stack: undefined as any,
        node: undefined as any,
      } as unknown as acm.ICertificate,
    });

    // Generate CloudFormation template
    template = Template.fromStack(stack);
  });

  test('S3 Bucket Created', () => {
    // Verify that an S3 bucket is created with the correct properties
    template.hasResourceProperties('AWS::S3::Bucket', {
      BucketName: 'test-example-com',
    });
  });

  test('CloudFront Distribution Created', () => {
    // Verify that a CloudFront distribution is created
    template.resourceCountIs('AWS::CloudFront::Distribution', 1);
  });

  test('CloudFront Function Created', () => {
    // Verify that a CloudFront function is created
    template.resourceCountIs('AWS::CloudFront::Function', 1);
  });

  test('Security Headers Policy Created', () => {
    // Verify that a response headers policy is created
    template.resourceCountIs('AWS::CloudFront::ResponseHeadersPolicy', 1);
  });

  test('CloudFront Origin Access Control Used', () => {
    // Verify that Origin Access Control is used for CloudFront to access S3
    template.resourceCountIs('AWS::CloudFront::OriginAccessControl', 1);
  });

  test('S3 Bucket Policy Created', () => {
    // Verify that a bucket policy is created to allow CloudFront access
    template.resourceCountIs('AWS::S3::BucketPolicy', 1);
  });

  test('Stack Outputs Created', () => {
    // Verify that the stack has the expected outputs
    template.hasOutput('BucketName', {});
    template.hasOutput('DistributionId', {});
    template.hasOutput('DistributionDomainName', {});
  });
});
