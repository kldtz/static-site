SHELL:=/bin/bash
.SHELLFLAGS:=-O extglob -c
MD_FILES = $(shell find private/content/ -type f -name '*.md')
HTML_FILES = $(patsubst private/content/%.md, public/%.html, $(MD_FILES))

.PHONY: all publish sync-static clean dev feed

all: sync-static index feed $(HTML_FILES)

changes: sync-static $(HTML_FILES)

sync-static:
	# We cannot sync the whole private dir with the delete flag
	# because it would remove all non-static files
	rsync -r --exclude ".git" private/static/* public/
	rsync -r --exclude "*.md" --exclude "*.draft*" private/content/* public/
	# Hidden files are ignored by the command above
	touch public/.nojekyll

public/%.html: private/content/%.md
	mkdir -p "$(@D)"
	# Use the latest script (compile if necessary)
	cargo run --release page "$<" > "$@"

dev:
	browser-sync start --server 'public' --files 'public'

clean:
	rm -rf public/!(.git)

feed:
	cargo run --release feed > public/rss.xml

index:
	cargo run --release index > public/index.html
