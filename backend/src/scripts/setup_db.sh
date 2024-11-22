#!/bin/bash
PGPASSWORD=ffball psql -U ffball -d ffball -f "src/database/setup_db.sql"
