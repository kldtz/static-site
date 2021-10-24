# Site Generator

Code for generating [my static site](https://proceed-to-decode.com/). The content lives in a submodule under `private` and is expected to have the following structure:

```bash
├── content
│   ├── index.md
│   └── posts
├── static
└── templates
    ├── default.html
    ├── top.html
    └── ...
```

The `content` directory contains all Markdown files. For each index file under `posts`, a link is created on the home page. All files under `static` are simply copied to the public directory (keeping the path relative to `static`), e.g., JavaScript, CSS, pictures. Under `templates` the script is looking for the [Askama](https://github.com/djc/askama) templates linked in the Markdown headers.

The website is written into the submodule `public`.

## Usage

```bash
# Create a new post
./bin/new "This is a title"

# Generate all files (including home page and feed)
make

# Run server and watch files in public dir
make dev
# Generate HTML from changed Markdown files
# (this is run on saving .md files in VSCode)
make changes
# Clean public dir
make clean

# Update RSS feed
make feed
# Update home page
make index

# Publish all changes with generic commit message
make publish
```
