
default: (run "cedict") (run "wiktionary" "eng")

run *args:
	cargo run --release -- {{args}}

build:
	cargo build --release
