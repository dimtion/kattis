#!/usr/bin/env bash

set -e

if [ -z $1 ]; then
        echo "usage: $0 <kattis_problem_name>"
        echo ""
        echo "Creates a new template for a kattis problem"
        exit 1
fi
if [ -d "$1" ]; then
        (>&2 echo "Directory $1 already exists, aborting!")
        exit 1
fi

SAMPLE_URL="https://open.kattis.com/problems/$1/file/statement/samples.zip"

mkdir "$1"
cp .template_main.rs "$1/main.rs"
cp .template_Makefile "$1/Makefile"

cd "$1"
curl -sO "$SAMPLE_URL"
unzip *.zip

echo "Done creating folder $1."
