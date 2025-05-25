import * as cdk from 'aws-cdk-lib';
import { Template } from 'aws-cdk-lib/assertions';
import * as cloudfront from 'aws-cdk-lib/aws-cloudfront';
import * as s3 from 'aws-cdk-lib/aws-s3';
import * as acm from 'aws-cdk-lib/aws-certificatemanager';
import { CloudFrontFunction, DistributionConstruct } from '../../lib/constructs';

describe('DistributionConstruct', () => {
  let stack: cdk.Stack;
  let template: Template;

  beforeEach(() => {
    // Create a new stack for each test
    stack = new cdk.Stack();

    // Create mock resources needed for the distribution
    const bucket = new s3.Bucket(stack, 'TestBucket');
    const certificate = new acm.Certificate(stack, 'TestCertificate', {
      domainName: 'test.example.com',
    });
    const responseHeadersPolicy = new cloudfront.ResponseHeadersPolicy(stack, 'TestPolicy', {
      responseHeadersPolicyName: 'test-policy',
      securityHeadersBehavior: {
        contentSecurityPolicy: {
          contentSecurityPolicy: "default-src 'self'",
          override: true,
        },
      },
    });
    const cloudfrontFunction = new CloudFrontFunction(stack, 'TestFunction', {
      domainName: 'test.example.com',
    });

    // Create the construct with test properties
    new DistributionConstruct(stack, 'TestDistribution', {
      domainName: 'test.example.com',
      certificate: certificate,
      bucket: bucket,
      responseHeadersPolicy: responseHeadersPolicy,
      cloudFrontFunction: cloudfrontFunction,
    });

    // Generate CloudFormation template
    template = Template.fromStack(stack);
  });

  test('CloudFront Distribution Created with Correct Properties', () => {
    // Verify that a CloudFront distribution is created
    template.resourceCountIs('AWS::CloudFront::Distribution', 1);

    // Verify distribution properties
    template.hasResourceProperties('AWS::CloudFront::Distribution', {
      DistributionConfig: {
        DefaultRootObject: 'index.html',
        Aliases: ['test.example.com'],
        DefaultCacheBehavior: {
          ViewerProtocolPolicy: 'redirect-to-https',
          FunctionAssociations: [
            {
              EventType: 'viewer-request',
              FunctionARN: {
                'Fn::GetAtt': ['TestFunction211AF449', 'FunctionARN'],
              },
            },
          ],
        },
        Enabled: true,
        HttpVersion: 'http2',
        IPV6Enabled: true,
        CustomErrorResponses: [
          {
            ErrorCode: 403,
            ResponseCode: 404,
            ResponsePagePath: '/404/index.html',
          },
        ],
      },
    });
  });

  test('Distribution Uses S3 Origin with Origin Access Control', () => {
    // Verify that the distribution uses an S3 origin with origin access control
    template.hasResourceProperties('AWS::CloudFront::Distribution', {
      DistributionConfig: {
        Origins: [
          {
            DomainName: {
              'Fn::GetAtt': ['TestBucket560B80BC', 'RegionalDomainName'],
            },
            S3OriginConfig: {
              OriginAccessIdentity: '',
            },
            OriginAccessControlId: {
              'Fn::GetAtt': ['TestDistributionOrigin1S3OriginAccessControl91BBACB1', 'Id'],
            },
          },
        ],
      },
    });
  });

  test('Origin Access Control is Created', () => {
    // Verify that an origin access control is created
    template.resourceCountIs('AWS::CloudFront::OriginAccessControl', 1);

    // Verify origin access control properties
    template.hasResourceProperties('AWS::CloudFront::OriginAccessControl', {
      OriginAccessControlConfig: {
        OriginAccessControlOriginType: 's3',
        SigningBehavior: 'always',
        SigningProtocol: 'sigv4',
      },
    });
  });

  test('Distribution Uses Certificate', () => {
    // Verify that the distribution uses the provided certificate
    template.hasResourceProperties('AWS::CloudFront::Distribution', {
      DistributionConfig: {
        ViewerCertificate: {
          AcmCertificateArn: {
            Ref: 'TestCertificate6B4956B6',
          },
          SslSupportMethod: 'sni-only',
          MinimumProtocolVersion: 'TLSv1.2_2021',
        },
      },
    });
  });

  test('Distribution Uses Response Headers Policy', () => {
    // Verify that the distribution uses the provided response headers policy
    template.hasResourceProperties('AWS::CloudFront::Distribution', {
      DistributionConfig: {
        DefaultCacheBehavior: {
          ResponseHeadersPolicyId: {
            Ref: 'TestPolicyCC05E598',
          },
        },
      },
    });
  });
});
