#!/bin/bash
psql -U neondb_owner -d neondb -f "src/database/setup_db.sql"
# PGPASSWORD=ffball psql -U ffball -d ffball -f "src/database/setup_db.sql"
