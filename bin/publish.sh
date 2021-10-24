#!/bin/bash

set -eo nounset

start_pwd=`pwd`
message=$1

function cleanup {
    cd $start_pwd
}

function publish {
    git add .
    git commit -m "$message"
    git push
}

cd private
publish

cd ../public
publish

trap cleanup EXIT