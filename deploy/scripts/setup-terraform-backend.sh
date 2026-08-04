#!/bin/bash

# Setup Terraform S3 Backend
# This script creates the S3 bucket and DynamoDB table needed for Terraform remote state management
# This is NOT for setting up the application backend - this is for Terraform infrastructure state

set -e

# Configuration
BUCKET_NAME="${1:-ffdraftboard-terraform-state}"
REGION="${2:-us-east-1}"
PROJECT_NAME="${3:-fantasy-football-draft-board}"

# Show help if requested
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
    echo "Terraform S3 Backend Setup Script"
    echo "=================================="
    echo ""
    echo "This script sets up S3 bucket and DynamoDB table for Terraform remote state management."
    echo "This is NOT for setting up the application backend."
    echo ""
    echo "Usage: $0 [bucket-name] [region] [project-name]"
    echo ""
    echo "Arguments:"
    echo "  bucket-name   S3 bucket name (default: ffdraftboard-terraform-state)"
    echo "  region        AWS region (default: us-east-1)"
    echo "  project-name  Project name (default: fantasy-football-draft-board)"
    echo ""
    echo "Examples:"
    echo "  $0                                    # Use defaults"
    echo "  $0 myproject-terraform-state         # Custom bucket name"
    echo "  $0 staging-terraform-state us-west-2 # Custom bucket and region"
    echo ""
    exit 0
fi

echo "🚀 Setting up Terraform S3 Backend for Infrastructure State..."
echo "Bucket: $BUCKET_NAME"
echo "Region: $REGION"
echo "Project: $PROJECT_NAME"
echo ""

# Check if AWS CLI is installed
if ! command -v aws &> /dev/null; then
    echo "❌ AWS CLI is not installed. Please install it first."
    exit 1
fi

# Check if AWS credentials are configured
if ! aws sts get-caller-identity &> /dev/null; then
    echo "❌ AWS credentials not configured. Please run 'aws configure' first."
    exit 1
fi

echo "✅ AWS credentials verified"

# Create S3 bucket
echo "📦 Creating S3 bucket: $BUCKET_NAME"
if aws s3 ls "s3://$BUCKET_NAME" 2>&1 | grep -q 'NoSuchBucket'; then
    aws s3 mb "s3://$BUCKET_NAME" --region "$REGION"
    echo "✅ S3 bucket created successfully"
else
    echo "ℹ️  S3 bucket already exists"
fi

# Enable versioning
echo "🔄 Enabling versioning..."
aws s3api put-bucket-versioning \
    --bucket "$BUCKET_NAME" \
    --versioning-configuration Status=Enabled

# Enable server-side encryption
echo "🔐 Enabling server-side encryption..."
aws s3api put-bucket-encryption \
    --bucket "$BUCKET_NAME" \
    --server-side-encryption-configuration '{
        "Rules": [
            {
                "ApplyServerSideEncryptionByDefault": {
                    "SSEAlgorithm": "AES256"
                }
            }
        ]
    }'

# Add bucket policy for additional security (optional)
echo "📋 Adding bucket policy..."
aws s3api put-bucket-policy \
    --bucket "$BUCKET_NAME" \
    --policy '{
        "Version": "2012-10-17",
        "Statement": [
            {
                "Sid": "DenyUnencryptedObjectUploads",
                "Effect": "Deny",
                "Principal": "*",
                "Action": "s3:PutObject",
                "Resource": "arn:aws:s3:::'$BUCKET_NAME'/*",
                "Condition": {
                    "StringNotEquals": {
                        "s3:x-amz-server-side-encryption": "AES256"
                    }
                }
            }
        ]
    }'

# Create DynamoDB table for state locking (optional but recommended)
TABLE_NAME="${BUCKET_NAME}-locks"
echo "🔒 Creating DynamoDB table for state locking: $TABLE_NAME"

if ! aws dynamodb describe-table --table-name "$TABLE_NAME" &> /dev/null; then
    aws dynamodb create-table \
        --table-name "$TABLE_NAME" \
        --attribute-definitions AttributeName=LockID,AttributeType=S \
        --key-schema AttributeName=LockID,KeyType=HASH \
        --billing-mode PAY_PER_REQUEST \
        --region "$REGION"
    
    echo "⏳ Waiting for DynamoDB table to be active..."
    aws dynamodb wait table-exists --table-name "$TABLE_NAME"
    echo "✅ DynamoDB table created successfully"
else
    echo "ℹ️  DynamoDB table already exists"
fi

echo ""
echo "🎉 Terraform S3 Backend setup complete!"
echo ""
echo "📋 Summary:"
echo "  • S3 Bucket: $BUCKET_NAME"
echo "  • DynamoDB Table: $TABLE_NAME"
echo "  • Region: $REGION"
echo ""
echo "💡 Next steps:"
echo "  1. Update your terraform.tfvars with the bucket name:"
echo "     backend_bucket = \"$BUCKET_NAME\""
echo "  2. Run 'terraform init -migrate-state' to migrate local state"
echo "  3. Run 'terraform plan' to verify everything works"
echo ""
echo "💰 Estimated monthly cost: ~$0.01-0.02" 