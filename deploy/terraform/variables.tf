variable "aws_region" {
  description = "AWS region"
  type        = string
  default     = "us-east-1"
}

variable "project_name" {
  description = "Name of the project"
  type        = string
  default     = "ffball"
}

variable "environment" {
  description = "Environment name"
  type        = string
  default     = "production"
}

variable "vpc_cidr" {
  description = "CIDR block for VPC"
  type        = string
  default     = "10.0.0.0/16"
}

variable "public_subnet_cidr" {
  description = "CIDR block for public subnet"
  type        = string
  default     = "10.0.1.0/24"
}

variable "instance_type" {
  description = "EC2 instance type"
  type        = string
  default     = "t3.medium"
}

variable "root_volume_size" {
  description = "Size of root volume in GB"
  type        = number
  default     = 20
}

variable "ssh_public_key_path" {
  description = "Path to SSH public key file"
  type        = string
  default     = "~/.ssh/ffball_deploy.pub"
}

variable "git_repo" {
  description = "Git repository URL"
  type        = string
  default     = "https://github.com/mengeling/fantasy-football-draft-board-v2.git"
}

variable "git_branch" {
  description = "Git branch to deploy"
  type        = string
  default     = "main"
  # TODO: Change back to "main" after config files are merged to main branch
}

variable "domain_name" {
  description = "Domain name for the application (optional)"
  type        = string
  default     = ""
}

variable "route53_zone_id" {
  description = "Route53 hosted zone ID (required if domain_name is set)"
  type        = string
  default     = ""
}

variable "backend_bucket" {
  description = "S3 bucket name for Terraform state backend"
  type        = string
  default     = "ffdraftboard-terraform-state"
}