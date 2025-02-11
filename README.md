# Ingenius Inventory Service

## Requirements

- Rust
- Podman
- PostgreSQL

## How to run

### Without podman

Make sure you have the .env file with the `DATABASE_URL` and then run

```bash
cargo run
```

### Using podman

Execute the script to create and run the service pod using podman

```bash
./podman-setup.sh
```

## Project structure

```bash
ingenius-inventory-service/
├── entity/                 # Sub-crate for entity-related functionality
│   ├── src/
│   │   ├── lib.rs          # Entry point for the entity crate (defines modules)
│   │   └── student.rs
│   ├── Cargo.toml
│   └── Cargo.lock
├── src/
│   └── main.rs             # Entry point of the application
├── Cargo.toml
├── Cargo.lock
├── Dockerfile              # Defines the Docker/Podman image for the application
├── .dockerignore
├── .gitignore
├── podman-setup.sh         # Script to set up and run the service using Podman
├── entrypoint.sh           # Script to run tasks when the container starts
└── README.md
```
