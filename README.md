# Cloud API
This is the backend of my own server.

## Proramming Languages
This backend is mainly written in rust, with several python scripts.

## Current Web APIs
### Genshin
`GET /genshin/info` for info about user's roles

`GET /genshin/time` for info about resin and other things related to time

## Dependencies
### Python
`pip3 install -U genshinhelper`
### Rust
cargo will automatically install dependencies

## Build
### Compile for my own server:
`sh build.sh`
### Compile for your device:
`cargo build --release`
