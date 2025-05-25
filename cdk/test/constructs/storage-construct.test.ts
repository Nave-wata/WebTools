import * as cdk from 'aws-cdk-lib';
import { Template } from 'aws-cdk-lib/assertions';
import * as cloudfront from 'aws-cdk-lib/aws-cloudfront';
import * as origins from 'aws-cdk-lib/aws-cloudfront-origins';
import { StorageConstruct } from '../../lib/constructs';

describe('StorageConstruct', () => {
  let stack: cdk.Stack;
  let template: Template;
  let storageConstruct: StorageConstruct;

  beforeEach(() => {
    // Create a new stack for each test
    stack = new cdk.Stack();

    // Create the construct with test properties
    storageConstruct = new StorageConstruct(stack, 'TestStorage', {
      bucketName: 'test.example.com',
    });

    // Generate CloudFormation template
    template = Template.fromStack(stack);
  });

  test('S3 Bucket Created with Correct Properties', () => {
    // Verify that an S3 bucket is created
    template.resourceCountIs('AWS::S3::Bucket', 1);

    // Verify bucket properties
    template.hasResourceProperties('AWS::S3::Bucket', {
      BucketEncryption: {
        ServerSideEncryptionConfiguration: [
          {
            ServerSideEncryptionByDefault: {
              SSEAlgorithm: 'AES256'
            }
          }
        ]
      },
      PublicAccessBlockConfiguration: {
        BlockPublicAcls: true,
        BlockPublicPolicy: true,
        IgnorePublicAcls: true,
        RestrictPublicBuckets: true
      },
    });
  });

  test('Bucket Name Format is Correct', () => {
    // Verify that the bucket name follows the expected pattern
    template.hasResourceProperties('AWS::S3::Bucket', {
      BucketName: 'test-example-com'
    });
  });

  test('CloudFront Access Permission', () => {
    // Create a new stack for this test to avoid multiple synth() calls
    const testStack = new cdk.Stack();
    const testStorageConstruct = new StorageConstruct(testStack, 'TestStorage', {
      bucketName: 'test.example.com',
    });

    // Create a mock CloudFront distribution
    const mockDistribution = new cloudfront.Distribution(testStack, 'MockDistribution', {
      defaultBehavior: {
        origin: new origins.HttpOrigin('example.com'),
      },
    });

    // Allow CloudFront access
    testStorageConstruct.allowCloudFrontAccess(mockDistribution);

    // Generate template from the new stack
    const testTemplate = Template.fromStack(testStack);

    // Verify that a bucket policy is created
    testTemplate.resourceCountIs('AWS::S3::BucketPolicy', 1);

    // Verify policy properties
    // Verify that a bucket policy exists with the expected CloudFront access statement
    const cfnTemplate = testTemplate.toJSON();
    const bucketPolicies = Object.values(cfnTemplate.Resources).filter(
      (r: any) => r.Type === 'AWS::S3::BucketPolicy'
    );

    expect(bucketPolicies.length).toBe(1);
    const bucketPolicy = bucketPolicies[0] as any;
    const statements = bucketPolicy.Properties.PolicyDocument.Statement;

    // Find the statement that allows CloudFront access
    const cloudfrontStatement = statements.find((s: any) => 
      s.Principal && 
      s.Principal.Service === 'cloudfront.amazonaws.com' && 
      s.Effect === 'Allow' && 
      s.Action === 's3:GetObject'
    );

    expect(cloudfrontStatement).toBeDefined();
    // Check that the AWS:SourceArn condition contains the expected CloudFront ARN pattern
    const sourceArn = cloudfrontStatement.Condition.StringEquals['AWS:SourceArn'];
    expect(sourceArn).toBeDefined();

    // If it's a string, check directly
    if (typeof sourceArn === 'string') {
      expect(sourceArn).toContain('arn:aws:cloudfront::');
    } 
    // If it's an object with Fn::Join, check the array elements
    else if (sourceArn['Fn::Join']) {
      const joinParts = sourceArn['Fn::Join'][1];
      expect(joinParts).toContain('arn:aws:cloudfront::');
    }
  });
});
