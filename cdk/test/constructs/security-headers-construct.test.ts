import * as cdk from 'aws-cdk-lib';
import { Template } from 'aws-cdk-lib/assertions';
import { SecurityHeadersConstruct } from '../../lib/constructs';

describe('SecurityHeadersConstruct', () => {
  let stack: cdk.Stack;
  let template: Template;

  beforeEach(() => {
    // Create a new stack for each test
    stack = new cdk.Stack();
    
    // Create the construct with test properties
    new SecurityHeadersConstruct(stack, 'TestSecurityHeaders', {
      policyName: 'test.example.com',
    });
    
    // Generate CloudFormation template
    template = Template.fromStack(stack);
  });

  test('Response Headers Policy Created with Correct Name', () => {
    // Verify that a response headers policy is created
    template.resourceCountIs('AWS::CloudFront::ResponseHeadersPolicy', 1);
    
    // Verify policy name
    template.hasResourceProperties('AWS::CloudFront::ResponseHeadersPolicy', {
      ResponseHeadersPolicyConfig: {
        Name: 'test-example-com_security-headers',
      },
    });
  });

  test('Content Security Policy is Configured', () => {
    // Verify that the Content Security Policy is configured correctly
    template.hasResourceProperties('AWS::CloudFront::ResponseHeadersPolicy', {
      ResponseHeadersPolicyConfig: {
        SecurityHeadersConfig: {
          ContentSecurityPolicy: {
            ContentSecurityPolicy: "default-src 'self'; img-src 'self' data:; script-src 'self' 'unsafe-inline' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; connect-src 'self';",
            Override: true,
          },
        },
      },
    });
  });

  test('Strict Transport Security is Configured', () => {
    // Verify that the Strict Transport Security is configured correctly
    template.hasResourceProperties('AWS::CloudFront::ResponseHeadersPolicy', {
      ResponseHeadersPolicyConfig: {
        SecurityHeadersConfig: {
          StrictTransportSecurity: {
            AccessControlMaxAgeSec: 2 * 365 * 24 * 60 * 60, // 2 years in seconds
            IncludeSubdomains: true,
            Preload: true,
            Override: true,
          },
        },
      },
    });
  });

  test('Content Type Options is Configured', () => {
    // Verify that the Content Type Options is configured correctly
    template.hasResourceProperties('AWS::CloudFront::ResponseHeadersPolicy', {
      ResponseHeadersPolicyConfig: {
        SecurityHeadersConfig: {
          ContentTypeOptions: {
            Override: true,
          },
        },
      },
    });
  });

  test('Frame Options is Configured', () => {
    // Verify that the Frame Options is configured correctly
    template.hasResourceProperties('AWS::CloudFront::ResponseHeadersPolicy', {
      ResponseHeadersPolicyConfig: {
        SecurityHeadersConfig: {
          FrameOptions: {
            FrameOption: 'DENY',
            Override: true,
          },
        },
      },
    });
  });

  test('XSS Protection is Configured', () => {
    // Verify that the XSS Protection is configured correctly
    template.hasResourceProperties('AWS::CloudFront::ResponseHeadersPolicy', {
      ResponseHeadersPolicyConfig: {
        SecurityHeadersConfig: {
          XSSProtection: {
            Protection: true,
            ModeBlock: true,
            Override: true,
          },
        },
      },
    });
  });

  test('Referrer Policy is Configured', () => {
    // Verify that the Referrer Policy is configured correctly
    template.hasResourceProperties('AWS::CloudFront::ResponseHeadersPolicy', {
      ResponseHeadersPolicyConfig: {
        SecurityHeadersConfig: {
          ReferrerPolicy: {
            ReferrerPolicy: 'strict-origin-when-cross-origin',
            Override: true,
          },
        },
      },
    });
  });
});
