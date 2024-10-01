@default:
    just --list

@run:
    cargo run --release

@run-debug:
    cargo run

@build:
    cargo build --release

@build-debug:
    cargo build
