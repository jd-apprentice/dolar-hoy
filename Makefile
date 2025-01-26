app_name ?= dolar-hoy

all: test lint format docker compose

start:
	./target/release/dolar-hoy

dev:
	cargo watch -x run

build: src/main.rs
	cargo build --release --bin dolar-hoy

docker:
	docker build -f docker/Dockerfile -t rust_template .

compose:
	docker compose -f docker/compose.yml up

test:
	cargo test

lint:
	cargo clippy --all-targets --all-features

lint-fix:
	cargo clippy --all-targets --all-features --fix --allow-dirty

format:
	cargo fmt --all --check

format-fix:
	cargo fmt --all

.PHONY: build docker test lint format compose
