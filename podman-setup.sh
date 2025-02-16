#!/bin/bash
set -e

# Create a network if it doesn't exist
if ! podman network exists ingenius-network; then
    echo "Creating network..."
    podman network create ingenius-network \
        --driver bridge \
        --opt dns_enabled=true
fi

# Remove existing pod if it exists
if podman pod exists ingenius-pod; then
    echo "Removing existing pod..."
    podman pod rm -f ingenius-pod
fi


# Remove existing volume if it exists
if podman volume exists ingenius-db-data; then
    echo "Removing existing volume..."
    podman volume rm -f ingenius-db-data
fi

# Create the pod with DNS settings
echo "Creating pod..."
podman pod create \
    --name ingenius-pod \
    --network ingenius-network \
    --share net \
    --publish 8080:8080 \
    --publish 5432:5432

# Start the database
echo "Starting database..."
podman run -d \
    --pod ingenius-pod \
    --name ingenius-db \
    -e POSTGRES_USER=postgres \
    -e POSTGRES_PASSWORD=postgres \
    -e POSTGRES_DB=ingenius \
    -v ingenius-db-data:/var/lib/postgresql/data:Z \
    postgres:15

# Wait for database to be ready
echo "Waiting for database to be ready..."
timeout=60
while [ $timeout -gt 0 ]; do
    if podman exec ingenius-db pg_isready -U postgres; then
        break
    fi
    echo "Database is not ready - waiting... ($timeout seconds left)"
    sleep 2
    timeout=$((timeout - 2))
done

if [ $timeout -le 0 ]; then
    echo "Timeout waiting for database to become ready"
    exit 1
fi

echo "Building service..."
podman build -t ingenius-service .

# Run ingenius-service with --replace
echo "Running new ingenius-service container..."
podman run -d --replace \
    --pod ingenius-pod \
    --name ingenius-service \
    --restart unless-stopped \
    -e DATABASE_URL=postgres://postgres:postgres@localhost:5432/ingenius \
    -e RUST_BACKTRACE=full \
    -e RUST_LOG=debug \
    ingenius-service

# Follow the logs with timeout
echo "Service logs (showing first 30 seconds):"
timeout 30 podman logs -f ingenius-service || true

# Check container status
status=$(podman container inspect -f '{{.State.Status}}' ingenius-service)
if [ "$status" != "running" ]; then
    echo "Container is not running (status: $status)"
    echo "Last few logs:"
    podman logs --tail 50 ingenius-service
    
    # Add network debugging
    echo "Network connectivity ingenius from ingenius-service:"
    podman exec -it ingenius-service nc -zv localhost 5432 || true
    
    exit 1
fi

echo "Service is running successfully!"
