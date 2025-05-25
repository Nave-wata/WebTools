import * as cdk from 'aws-cdk-lib';
import { Template } from 'aws-cdk-lib/assertions';
import * as route53 from 'aws-cdk-lib/aws-route53';
import { ResourcesReferenceConstruct } from '../../lib/constructs';
import { Construct } from 'constructs';

// Mock the HostedZone.fromLookup method
jest.mock('aws-cdk-lib/aws-route53', () => {
  const originalModule = jest.requireActual('aws-cdk-lib/aws-route53');
  return {
    ...originalModule,
    HostedZone: {
      ...originalModule.HostedZone,
      fromLookup: jest.fn((scope: Construct, id: string, props: route53.HostedZoneProviderProps) => {
        return new originalModule.HostedZone(scope, id, {
          zoneName: props.domainName,
        });
      }),
    },
  };
});

describe('ResourcesReferenceConstruct', () => {
  let stack: cdk.Stack;
  let template: Template;

  beforeEach(() => {
    // Create a new stack for each test
    stack = new cdk.Stack();
    
    // Create the construct with test properties
    new ResourcesReferenceConstruct(stack, 'TestResourcesReference', {
      zoneName: 'example.com',
    });
    
    // Generate CloudFormation template
    template = Template.fromStack(stack);
  });

  test('Hosted Zone is Looked Up by Domain Name', () => {
    // Verify that HostedZone.fromLookup was called with the correct parameters
    expect(route53.HostedZone.fromLookup).toHaveBeenCalledWith(
      expect.anything(),
      'HostedZone',
      {
        domainName: 'example.com',
      }
    );
  });

  test('Hosted Zone is Available as a Property', () => {
    // Create the construct again to access its properties
    const resourcesReferenceConstruct = new ResourcesReferenceConstruct(stack, 'TestResourcesReference2', {
      zoneName: 'example.com',
    });
    
    // Verify that the hosted zone is available as a property
    expect(resourcesReferenceConstruct.hostedZone).toBeDefined();
    expect(resourcesReferenceConstruct.hostedZone.zoneName).toBe('example.com');
  });
});
