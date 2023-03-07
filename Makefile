.PHONY: release

release:
	cargo build --release 
	cp target/release/pos-tagger ./pos-tagger

clean:
	cargo clean
	rm pos-tagger
