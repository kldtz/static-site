---
title: "A Minimalist Static Site Generator"
date: 2020-08-25T10:00:00+02:00
description: "Switching from Hugo to a self-hacked solution for generating this static site."
---

So far I have been using [Hugo](https://gohugo.io/) with a customized version of the [Hermit](https://github.com/Track3/hermit) theme to generate this site. This worked well and looked nice. However, Hugo is a relatively complex framework that keeps evolving (occasionally breaking stuff on updates) and the theme consists of a lot of files with styles, templates and scripts that I needed to maintain by myself since I customized them (in a not so clean way). Overall it felt like overkill for my purposes and being on vacation, I hacked together [my own minimalist site generator](https://github.com/kldtz/static-site) that suits my needs.

As the basis I use a Makefile that copies static files and calls a Rust script on each Markdown file to transform it into HTML with [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark). Each file is self-contained (well, except for internal links) such that only modified files need to be considered. The Markdown files consist of a header with meta data and the actual Markdown. The header declares a template which will be used to transform the file (if no template is declared, a default will be used). The content and optionally all data declared in the header can be used to fill the template, e.g., page title, creation date, script and style paths, and predefined keywords that enable features such as math typesetting with [MathJax](https://www.mathjax.org/) and syntax highlighting with [highlight.js](https://highlightjs.org/).

Writing and editing is the same as before, but now I have full control of the generation process, don't need to read any manuals, and can get by with a bare minimum of CSS and JavaScript.