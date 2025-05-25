import * as cdk from 'aws-cdk-lib';
import { Match, Template } from 'aws-cdk-lib/assertions';
import * as route53 from 'aws-cdk-lib/aws-route53';
import { CertificateConstruct } from '../../lib/constructs';

describe('CertificateConstruct', () => {
  let stack: cdk.Stack;
  let template: Template;

  beforeEach(() => {
    // Create a new stack for each test
    stack = new cdk.Stack();

    // Create a mock hosted zone for testing
    const hostedZone = new route53.HostedZone(stack, 'TestHostedZone', {
      zoneName: 'example.com',
    });

    // Create the construct with test properties
    new CertificateConstruct(stack, 'TestCertificate', {
      domainName: 'test.example.com',
      hostedZone: hostedZone,
    });

    // Generate CloudFormation template
    template = Template.fromStack(stack);
  });

  test('ACM Certificate Created with Correct Properties', () => {
    // Verify that an ACM certificate is created
    template.resourceCountIs('AWS::CertificateManager::Certificate', 1);

    // Verify certificate properties
    template.hasResourceProperties('AWS::CertificateManager::Certificate', {
      DomainName: 'test.example.com',
      ValidationMethod: 'DNS',
      KeyAlgorithm: 'RSA_2048',
    });
  });

  test('Certificate Uses DNS Validation with Hosted Zone', () => {
    // Verify that the certificate uses DNS validation with the provided hosted zone
    template.hasResourceProperties('AWS::CertificateManager::Certificate', {
      DomainValidationOptions: [
        {
          DomainName: 'test.example.com',
          HostedZoneId: Match.objectLike({
            Ref: Match.stringLikeRegexp('TestHostedZone')
          })
        }
      ]
    });
  });
});
