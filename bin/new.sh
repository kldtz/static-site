#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "Illegal number of parameters. Invocation: new \"This is a title\""
    exit 1
fi

title=$1
dir_name=`echo "${title}" | tr '[:upper:]' '[:lower:]' | tr ' ' '-'`
md_path="private/content/posts/`date +'%Y'`/${dir_name}"
mkdir -p $md_path
printf -- "---\ntitle: \"%s\"\ndate: `date --iso-8601=seconds`\ndescription: \"%s\"\n---" "$title" "$title" > "${md_path}/index.md"
