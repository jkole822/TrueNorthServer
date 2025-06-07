#!/bin/sh
set -e

echo "🗃️ Running database migrations..."
sqlx migrate run

echo "🚀 Starting TrueNorth server..."
exec ./true_north_server
