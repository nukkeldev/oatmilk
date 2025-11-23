#!/bin/sh
# Sets up the offline database for SQLx.

DATABASE_URL="sqlite://$(pwd)/.sqlx/dev.db"
echo "Offline Database: ${DATABASE_URL}"

echo -e "DATABASE_URL=${DATABASE_URL}" > .env

sqlx db create
sqlx migrate run
cargo sqlx prepare