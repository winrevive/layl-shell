set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

default:
    just --list

@run:
    cargo run --release -- $ARGS