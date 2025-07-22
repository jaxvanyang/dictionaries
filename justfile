prefix := "/usr"

default: (run "cedict") (run "wiktionary" "eng")

run *args:
	cargo run --release -- {{args}}

build:
	cargo build --release

install:
	install -Dm655 out/cedict/zho-eng.odict "{{prefix}}/share/mydict/CC-CEDICT.odict"
	install -Dm655 "out/wiktionary/eng.odict" "{{prefix}}/share/mydict/English Wiktionary.odict"

uninstall:
	rm -f "{{prefix}}/share/mydict/CC-CEDICT.odict"
	rm -f "{{prefix}}/share/mydict/English Wiktionary.odict"
