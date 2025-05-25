import * as cdk from 'aws-cdk-lib';
import { Template } from 'aws-cdk-lib/assertions';
import { CloudFrontFunction } from '../../lib/constructs';
import * as fs from 'fs';
import * as path from 'path';

describe('CloudfrontFunction', () => {
  let stack: cdk.Stack;
  let template: Template;

  beforeEach(() => {
    // Create a new stack for each test
    stack = new cdk.Stack();

    // Create the construct with test properties
    new CloudFrontFunction(stack, 'TestFunction', {
      domainName: 'test.example.com',
    });

    // Generate CloudFormation template
    template = Template.fromStack(stack);
  });

  test('CloudFront Function Created with Correct Properties', () => {
    // Verify that a CloudFront function is created
    template.resourceCountIs('AWS::CloudFront::Function', 1);

    // Verify function properties
    template.hasResourceProperties('AWS::CloudFront::Function', {
      FunctionConfig: {
        Comment: 'Denies access to the default CloudFront distribution domain. And appends index.html to URIs that end with / or have no file extension.',
        Runtime: 'cloudfront-js-2.0'
      },
      Name: 'test-example-com_viewer-request'
    });
  });

  test('Function Code is Loaded from File', () => {
    // Verify that the function code is loaded from the correct file
    fs.readFileSync(
      path.join(__dirname, '../../lib/functions/cloudfront/viewer-request.js'),
      'utf8'
    );

    // The code should be included in the template
    // The code is a string in the CloudFormation template
    const cfnTemplate = template.toJSON();
    const functionResources = Object.values(cfnTemplate.Resources).filter(
      (r: any) => r.Type === 'AWS::CloudFront::Function'
    );

    expect(functionResources.length).toBe(1);
    const functionResource = functionResources[0] as any;
    expect(functionResource.Properties.FunctionCode).toContain('function handler(event)');
  });
});

// Test the actual function logic
describe('ViewerRequestFunction', () => {
  // Load the function code
  const functionPath = path.join(__dirname, '../../lib/functions/cloudfront/viewer-request.js');
  const functionCode = fs.readFileSync(functionPath, 'utf8');

  // Create a function from the code
  // Note: This is a simplified approach and doesn't fully simulate the CloudFront function environment
  const viewerRequestFunction = new Function(
    'event',
    functionCode + '\nreturn handler(event);'
  );

  test('Blocks Access to CloudFront Domain', () => {
    const event = {
      request: {
        uri: '/path',
        headers: {
          host: { value: 'distribution-id.cloudfront.net' }
        }
      }
    };

    const result = viewerRequestFunction(event);

    expect(result.statusCode).toBe(403);
    expect(result.statusDescription).toBe('Forbidden');
    expect(result.body).toBe('Access denied');
  });

  test('Appends index.html to URI Ending with Slash', () => {
    const event = {
      request: {
        uri: '/path/',
        headers: {
          host: { value: 'test.example.com' }
        }
      }
    };

    const result = viewerRequestFunction(event);

    expect(result.uri).toBe('/path/index.html');
  });

  test('Appends /index.html to URI Without Extension', () => {
    const event = {
      request: {
        uri: '/path',
        headers: {
          host: { value: 'test.example.com' }
        }
      }
    };

    const result = viewerRequestFunction(event);

    expect(result.uri).toBe('/path/index.html');
  });

  test('Leaves URI with Extension Unchanged', () => {
    const event = {
      request: {
        uri: '/path/file.jpg',
        headers: {
          host: { value: 'test.example.com' }
        }
      }
    };

    const result = viewerRequestFunction(event);

    expect(result.uri).toBe('/path/file.jpg');
  });
});
