BINDIR = /usr/bin
LICENSESDIR = /usr/share/licenses
DOCDIR = /usr/share/doc
TARGET = lsx

all: clean build install

build:
	cargo build --release --verbose

clean:
	cargo clean --verbose
	rm -fv $(BINDIR)/$(TARGET)

install:
	install -D -m755 -v ./target/release/$(TARGET) $(BINDIR)/$(TARGET)
	install -D -m655 -v ./README.md $(DOCDIR)/$(TARGET)/README.md
	install -D -m655 -v ./LICENSE $(LICENSESDIR)/$(TARGET)/LICENSE

uninstall:
	rm -fv $(BINDIR)/$(TARGET)
	rm -fv $(DOCDIR)/$(TARGET)
	rm -fv $(LICENSESDIR)/$(TARGET)

.PHONY: all build clean install uninstall
