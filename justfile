default: (run "wiktionary" "eng")

# Run a specific conversion
run generator lang:
	cargo run --release -- {{generator}} {{lang}}

build:
	cargo build --release
