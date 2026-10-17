# XDG by default. PREFIX selects a prefix. DESTDIR stages and skips man setup.
root := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

ifeq ($(PREFIX),)
  BINDIR ?= $(HOME)/.local/bin
  MANDIR ?= $(or $(XDG_DATA_HOME),$(HOME)/.local/share)/man
else
  BINDIR ?= $(PREFIX)/bin
  MANDIR ?= $(PREFIX)/share/man
endif

.PHONY: all build install uninstall clean test

all: build

build:
	cargo build --release

install: build
	install -D -m 755 target/release/conv "$(DESTDIR)$(BINDIR)/conv"
	install -D -m 644 doc/conv.1 "$(DESTDIR)$(MANDIR)/man1/conv.1"
ifeq ($(DESTDIR),)
	sh "$(root)scripts/man-install" install "$(BINDIR)" "$(MANDIR)"
endif

uninstall:
	rm -f "$(DESTDIR)$(BINDIR)/conv"
	rm -f "$(DESTDIR)$(MANDIR)/man1/conv.1"
ifeq ($(DESTDIR),)
	-mandb -q "$(MANDIR)"
	sh "$(root)scripts/man-install" uninstall "$(BINDIR)" "$(MANDIR)"
endif
	rmdir "$(DESTDIR)$(MANDIR)/man1" 2>/dev/null || true
	@dir="$(DESTDIR)$(MANDIR)"; \
	if [ -d "$$dir" ] && [ -z "$$(find "$$dir" -mindepth 2 -type f -path '*/man*/*' -print -quit)" ]; then \
		rm -f "$$dir/index.db" "$$dir/index.bt" "$$dir/index.dir" "$$dir/index.pag"; \
		rmdir "$$dir/cat1" "$$dir" 2>/dev/null || true; \
	fi

clean:
	cargo clean

test:
	cargo test
