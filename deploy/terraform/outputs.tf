output "public_ip" {
  description = "Public IP address of the web server"
  value       = aws_eip.web.public_ip
}

output "private_ip" {
  description = "Private IP address of the web server"
  value       = aws_instance.web.private_ip
}

output "instance_id" {
  description = "ID of the EC2 instance"
  value       = aws_instance.web.id
}

output "security_group_id" {
  description = "ID of the security group"
  value       = data.aws_security_group.existing.id
}

output "vpc_id" {
  description = "ID of the VPC"
  value       = data.aws_vpc.default.id
}

output "subnet_id" {
  description = "ID of the public subnet"
  value       = data.aws_subnet.default.id
}

output "ssh_command" {
  description = "SSH command to connect to the instance"
  value       = "ssh -i ~/.ssh/ffball_deploy ubuntu@${aws_eip.web.public_ip}"
}

output "application_url" {
  description = "URL to access the application"
  value       = var.domain_name != "" ? "https://${var.domain_name}" : "http://${aws_eip.web.public_ip}"
}