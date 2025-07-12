#!/bin/bash

# Setup development database
# This script should be run on the EC2 instance

set -e

echo "Setting up development database..."

# Set environment variable for dev database
export DATABASE_URL="postgresql://ffball:ffball@localhost/ffball_dev"

# Run the database setup script
cd /home/ubuntu/fantasy-football-draft-board-v2/backend
PGPASSWORD=ffball psql -U ffball -d ffball_dev -f "src/database/setup_db.sql"

echo "Development database setup complete!" 