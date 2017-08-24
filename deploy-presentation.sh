#! /usr/bin/env bash

set -o errexit -o nounset

if [ -f ~/.bash_tokens ]; then
    source ~/.bash_tokens
fi

rev=$(git rev-parse --short HEAD)

rm -rf deploy
mkdir deploy
cp index.html deploy/
cp -r resources deploy/
cp -r bower_components deploy/

cd deploy

git init
git config user.name "Daan van Berkel"
git config user.email "daan@fifth-postulate.nl"

git remote add upstream "https://$GH_TOKEN@github.com/fifth-postulate/rustfest.zurich.git"
git fetch upstream
git reset upstream/gh-pages

touch .
touch .nojekyll

git add -A .
git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
