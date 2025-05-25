import * as cdk from 'aws-cdk-lib';
import { Match, Template } from 'aws-cdk-lib/assertions';
import * as route53 from 'aws-cdk-lib/aws-route53';
import * as cloudfront from 'aws-cdk-lib/aws-cloudfront';
import * as origins from 'aws-cdk-lib/aws-cloudfront-origins';
import * as s3 from 'aws-cdk-lib/aws-s3';
import { DnsRecordConstruct } from '../../lib/constructs';

describe('DnsRecordConstruct', () => {
  let stack: cdk.Stack;
  let template: Template;

  beforeEach(() => {
    // Create a new stack for each test
    stack = new cdk.Stack();

    // Create mock resources needed for the DNS record
    const hostedZone = new route53.HostedZone(stack, 'TestHostedZone', {
      zoneName: 'example.com',
    });

    const bucket = new s3.Bucket(stack, 'TestBucket');
    const distribution = new cloudfront.Distribution(stack, 'TestDistribution', {
      defaultBehavior: {
        origin: origins.S3BucketOrigin.withOriginAccessControl(bucket),
      },
    });

    // Create the construct with test properties
    new DnsRecordConstruct(stack, 'TestDnsRecord', {
      hostedZone: hostedZone,
      distribution: distribution,
      recordName: 'test',
    });

    // Generate CloudFormation template
    template = Template.fromStack(stack);
  });

  test('Route53 A Record Created with Correct Properties', () => {
    // Verify that a Route53 A record is created
    template.resourceCountIs('AWS::Route53::RecordSet', 1);

    // Verify record properties
    template.hasResourceProperties('AWS::Route53::RecordSet', {
      Name: 'test.example.com.',
      Type: 'A',
      HostedZoneId: Match.objectLike({ Ref: Match.stringLikeRegexp('TestHostedZone') }),
      AliasTarget: Match.objectLike({
        DNSName: Match.anyValue(),
        HostedZoneId: Match.anyValue()
      }),
    });
  });
  
  test('Record is an Alias to CloudFront Distribution', () => {
    // Get the template as a JSON object
    const templateJson = template.toJSON();
    
    // Find the Route53 record in the resources
    const resources = templateJson.Resources;
    let recordFound = false;
    
    // Check each resource
    for (const resourceId in resources) {
      const resource = resources[resourceId];
      if (resource.Type === 'AWS::Route53::RecordSet') {
        // Verify the resource has an AliasTarget with DNSName and HostedZoneId
        expect(resource.Properties.AliasTarget).toBeDefined();
        expect(resource.Properties.AliasTarget.DNSName).toBeDefined();
        expect(resource.Properties.AliasTarget.HostedZoneId).toBeDefined();
        recordFound = true;
        break;
      }
    }
    
    // Ensure we found the record
    expect(recordFound).toBeTruthy();
  });
});
