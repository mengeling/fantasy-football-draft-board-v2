terraform {
  required_version = ">= 1.0"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

provider "aws" {
  region = var.aws_region
}

# Data sources
data "aws_availability_zones" "available" {
  state = "available"
}

# Use default VPC
data "aws_vpc" "default" {
  default = true
}

# Use existing subnet in us-east-1a
data "aws_subnet" "default" {
  id = "subnet-a5be788a"
}

# Use a specific Ubuntu 22.04 AMI ID for us-east-1
# This AMI ID is for Ubuntu 22.04 LTS (Jammy Jellyfish) in us-east-1
data "aws_ami" "ubuntu" {
  most_recent = true
  owners      = ["099720109477"] # Canonical

  filter {
    name   = "name"
    values = ["ubuntu/images/hvm-ssd/ubuntu-jammy-22.04-amd64-server-*"]
  }

  filter {
    name   = "virtualization-type"
    values = ["hvm"]
  }

  filter {
    name   = "architecture"
    values = ["x86_64"]
  }

  filter {
    name   = "root-device-type"
    values = ["ebs"]
  }
}

# Using default VPC and subnet - no need to create networking resources

# Use existing security group
data "aws_security_group" "existing" {
  id = "sg-f5b4f781"
}

# Create project-specific key pair
resource "aws_key_pair" "deploy" {
  key_name   = "${var.project_name}-deploy-key"
  public_key = file(pathexpand(var.ssh_public_key_path))
}

# EC2 Instance
resource "aws_instance" "web" {
  ami                    = data.aws_ami.ubuntu.id
  instance_type          = var.instance_type
  key_name               = aws_key_pair.deploy.key_name
  vpc_security_group_ids = [data.aws_security_group.existing.id]
  subnet_id              = data.aws_subnet.default.id

  root_block_device {
    volume_type = "gp3"
    volume_size = var.root_volume_size
    encrypted   = true
  }

  user_data = templatefile("${path.module}/user_data.sh", {
    project_name = var.project_name
    environment  = var.environment
    git_repo     = var.git_repo
    git_branch   = var.git_branch
    domain_name  = var.domain_name
  })

  tags = {
    Name        = "${var.project_name}-web-server"
    Environment = var.environment
  }
}

# Elastic IP
resource "aws_eip" "web" {
  instance = aws_instance.web.id
  domain   = "vpc"

  tags = {
    Name        = "${var.project_name}-eip"
    Environment = var.environment
  }
}

# Route 53 Record (optional)
resource "aws_route53_record" "web" {
  count   = var.domain_name != "" ? 1 : 0
  zone_id = var.route53_zone_id
  name    = var.domain_name
  type    = "A"
  ttl     = 300
  records = [aws_eip.web.public_ip]
}