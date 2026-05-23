BINARY   := ma
CARGO    := cargo
RELEASE  := target/release/$(BINARY)
DEBUG    := target/debug/$(BINARY)
CLIPPY_STRICT := --all-targets --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery
SRCS     := Cargo.toml src/i18n.yaml $(shell find src -name '*.rs')
PREFIX   ?= $(HOME)/.local/bin
PUBLISH  := ma:bin/
RUN_ARGS ?=

.PHONY: all clean distclean install lint publish test

all: $(BINARY)

lint:
	$(CARGO) clippy -- -D warnings
	$(CARGO) fmt --check
	mdl *.md

test:
	$(CARGO) clippy $(CLIPPY_STRICT)

# Publish all i18n/*.ftl files to IPFS and write the resulting CIDs to
# src/i18n.yaml.  Requires `ipfs` (Kubo) and `jq` to be available.
# This file is a build input: `make release` will rebuild the binary
# whenever any FTL file changes.
src/i18n.yaml: $(wildcard i18n/*.ftl)
	@set -e; \
	dag='{}'; \
	for f in i18n/*.ftl; do \
		code=$$(basename $$f .ftl); \
		cid=$$(ipfs add -q --cid-version 1 "$$f"); \
		dag=$$(printf '%s' "$$dag" | jq --arg k "$$code" --arg v "$$cid" '. + {($$k): {"/": $$v}}'); \
	done; \
	lang_cid=$$(printf '%s' "$$dag" | ipfs dag put --input-codec dag-json --store-codec dag-cbor); \
	{ \
		printf 'i18n_cid: %s\n' "$$lang_cid"; \
		printf 'langs:\n'; \
		printf '%s' "$$dag" | jq -r 'to_entries[] | "  " + .key + ": " + .value["/"]'; \
	} > src/i18n.yaml; \
	echo "Written src/i18n.yaml (i18n_cid=$$lang_cid)"

gen-kinds-cids:
$(BINARY): $(RELEASE)
	cp $(RELEASE) $(BINARY)

$(RELEASE): $(SRCS)
	$(CARGO) build --release

clean:
	$(CARGO) clean
	rm -f $(BINARY)

install: $(RELEASE)
	mkdir -p $(PREFIX)
	install -m 0755 $(RELEASE) $(PREFIX)/$(BINARY)

publish: $(BINARY)
	scp $(BINARY) $(PUBLISH)

distclean: clean
	rm -rf target
	rm -rf Cargo.lock src/i18n.yaml
