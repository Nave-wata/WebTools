import * as cdk from 'aws-cdk-lib';
import { Template } from 'aws-cdk-lib/assertions';
import { BaseInfrastructureStack } from '../../lib/stacks';

describe('BaseInfrastructureStack', () => {
  let app: cdk.App;
  let stack: BaseInfrastructureStack;
  let template: Template;

  beforeEach(() => {
    // Create a new app for each test
    app = new cdk.App();
    
    // Create the stack with test properties
    stack = new BaseInfrastructureStack(app, 'TestBaseInfraStack', {
      env: { account: '123456789012', region: 'us-east-1' },
      zoneName: 'example.com',
      domainName: 'test.example.com',
    });
    
    // Generate CloudFormation template
    template = Template.fromStack(stack);
  });

  test('Certificate is Created', () => {
    // Verify that an ACM certificate is created
    template.resourceCountIs('AWS::CertificateManager::Certificate', 1);
    
    // Verify certificate properties
    template.hasResourceProperties('AWS::CertificateManager::Certificate', {
      DomainName: 'test.example.com',
      ValidationMethod: 'DNS'
    });
  });

  test('Stack Outputs Created', () => {
    // Verify that the stack has the expected outputs
    template.hasOutput('CertificateArn', {});
    template.hasOutput('HostedZoneId', {});
  });

  test('ResourcesReferenceConstruct Imports Hosted Zone', () => {
    // This is a bit tricky to test since it's importing an existing resource
    // We can verify that no new hosted zone is created
    template.resourceCountIs('AWS::Route53::HostedZone', 0);
  });
});
