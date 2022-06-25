SHELL:=/bin/bash
.SHELLFLAGS:=-O extglob -c
MD_FILES = $(shell find private/content/ -type f -name '*.md')
HTML_FILES = $(patsubst private/content/%.md, public/%.html, $(MD_FILES))

.PHONY: all publish sync-static clean dev feed

# Generate all files (including home page and feed)
all: sync-static index feed $(HTML_FILES)

# Generate HTML from changed Markdown files
# (this is run on saving .md files in VSCode)
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
	ssg page "$<" > "$@"

dev:
	browser-sync start --server 'public' --files 'public'

# Clean public dir
clean:
	rm -rf public/!(.git)

# Update RSS feed
feed:
	ssg feed > public/rss.xml

# Update home page
index:
	ssg index > public/index.html

# Publish all changes with generic commit message
publish:
	./bin/publish.sh "Update"

# Build site generator
build:
	cargo build --release