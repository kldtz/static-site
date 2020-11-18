SHELL:=/bin/bash
.SHELLFLAGS:=-O extglob -c
MD_FILES = $(shell find private/content/ -type f -name '*.md')
HTML_FILES = $(patsubst private/content/%.md, public/%.html, $(MD_FILES))

.PHONY: all sync-static deploy clean dev feed

all: sync-static index $(HTML_FILES) #deploy

sync-static:
	# We cannot sync the whole private dir with the delete flag
	# because it would remove all non-static files
	rsync -r --exclude ".git" private/static/* public/ --delete
	# Hidden files are ignored by the command above
	touch public/.nojekyll

public/%.html: private/content/%.md
	mkdir -p "$(@D)"
	# Use the latest script (compile if necessary)
	cargo run --release page "$<" > "$@"

deploy:
	rsync -r public/ /var/www/html/ --delete

dev:
	cd public && browser-sync start --server --files .

clean:
	rm -rf public/!(.git)

feed:
	cargo run --release feed > public/rss.xml

index:
	cargo run --release index > public/index.html
