FROM node:22-slim

# Install AWS CDK globally
RUN npm install -g aws-cdk

# Install AWS CLI using the official installer
RUN apt-get update  \
    && apt-get install -y --no-install-recommends \
        curl \
        unzip \
        ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip" \
    && unzip awscliv2.zip \
    && ./aws/install \
    && rm -rf aws awscliv2.zip

# Verify installations
RUN cdk --version && aws --version

# Set working directory
WORKDIR /app

# Default command
CMD ["bash"]
