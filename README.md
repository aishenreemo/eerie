## Eerie
A ~~multi-purpose~~ discord bot written in rust.

## Running a new instance of Eerie
```sh
git clone https://github.com/aishenreemo/eerie
cd eerie
cp .env.example .env # edit the configuration there or else it won't work

cargo clippy --all --all-targets -- -D warnings && cargo fmt && cargo run
```
