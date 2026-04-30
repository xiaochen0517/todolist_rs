#!/bin/bash
# Re-initialize Database Script (Linux/macOS)
# Function: Delete PostgreSQL data volume and restart container with init scripts

set -e

echo "Starting database reinitialization process..."
echo ""

# Stop containers and delete data volume
echo "Removing existing containers and volumes..."
docker-compose down -v

echo ""
echo "Starting new containers and initializing database..."
docker-compose up -d

echo ""
echo "Waiting for database initialization to complete (5 seconds)..."
sleep 5

echo ""
echo "[SUCCESS] Database re-initialization completed!"
echo ""
docker-compose ps

