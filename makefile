# Makefile for True North Server project

# Variables
BINARY_NAME := true_north_server
DOCKER_IMAGE := truenorthserver
DOCKER_COMPOSE := docker-compose

# Default target
.PHONY: all
all: build

# Local dev build
.PHONY: build
build:
	cargo build

# Release build
.PHONY: release
release:
	cargo build --release

# Format code
.PHONY: fmt
fmt:
	cargo fmt

# Run locally
.PHONY: run
run:
	cargo run

# Clean build
.PHONY: clean
clean:
	cargo clean

# Run Docker Compose stack
.PHONY: up
up:
	$(DOCKER_COMPOSE) up --build

# Tear down Docker Compose stack
.PHONY: down
down:
	$(DOCKER_COMPOSE) down

# View container logs
.PHONY: logs
logs:
	$(DOCKER_COMPOSE) logs -f

# Enter app container
.PHONY: shell
shell:
	$(DOCKER_COMPOSE) exec app sh

# Run SQLx prepare command
.PHONY: prepare
prepare:
	DATABASE_URL=postgres://$(POSTGRES_USER):$(POSTGRES_PASSWORD)@localhost:5432/$(POSTGRES_DB) \
	cargo sqlx prepare -- --bin $(BINARY_NAME)
