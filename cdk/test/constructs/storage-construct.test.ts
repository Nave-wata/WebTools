import * as cdk from 'aws-cdk-lib';
import { Template } from 'aws-cdk-lib/assertions';
import { StorageConstruct } from '../../lib/constructs/storage/static-site';

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

});
