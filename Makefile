BINARY   := ma
CARGO    := cargo
RELEASE  := target/release/$(BINARY)
DEBUG    := target/debug/$(BINARY)
CLIPPY_STRICT := --all-targets --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery
SRCS     := Cargo.toml $(shell find src -name '*.rs')

.PHONY: all clean distclean lint publish test

all: $(BINARY)

lint:
	$(CARGO) clippy -- -D warnings
	$(CARGO) fmt --check
	mdl *.md

test:
	$(CARGO) clippy $(CLIPPY_STRICT)

# Release build, binary copied to project root
$(BINARY): $(RELEASE)
	cp $(RELEASE) $(BINARY)

$(RELEASE): $(SRCS)
	$(CARGO) build --release

clean:
	$(CARGO) clean
	rm -f $(BINARY)

publish: $(BINARY)
	scp $(BINARY) ma:bin/

distclean: clean
	rm -rf Cargo.lock
