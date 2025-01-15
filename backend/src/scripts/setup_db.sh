#!/bin/bash
psql -U neondb_owner -d neondb -f "src/database/setup_db.sql"
