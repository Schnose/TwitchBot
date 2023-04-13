db:
  docker-compose up -d
  sleep 1
  PGPASSWORD=postgres ./migrations/connect.example.sh < ./migrations/schemas_up.sql

# Run locally
run:
  # Run the bot
  cargo shuttle run --port 9000

# Connect to local database
connect:
  ./migrations/connect.example.sh

# Setup local environment
dev:
  just db
  just run

# Clean up (THIS WILL DELETE VOLUMES)
yeet:
  docker-compose down -v

# Deploy to shuttle.rs
prod:
  cargo shuttle deploy
