#!/bin/env sh

genrefs_bin=$(realpath "$(dirname $0)/target/release/parse-references")

cd "$1"

dst_dir="/home/gabriel/Projects/revistagalo/content/edições/edição 00$2"

set -e

genrefs() {
  refs="$1"
  
  cat "$refs" | RUST_BACKTRACE=1 "$genrefs_bin"
}

for d in $(ls -d ./{articles,tex}/*); do
  if ! [ -d "${d}" ]; then
    echo "${d} is not a directory"
    continue
  fi

  article_name=$(basename $d | sed 's/-/ /g' | sed 's/ \+$//g')

  site_article="${dst_dir}/${article_name}/index.md"
  references_file="${d}/references.bib"

  if ! [ -f "${references_file}" ]; then
    echo "reference file ${references_file} does not exists"
    continue
  fi

  if ! [ -f "${site_article}" ]; then
    article_name=$(basename $d | sed 's/-\+$//g')

    site_article="${dst_dir}/${article_name}/index.md"

    if ! [ -f "${site_article}" ]; then
      echo "file ${site_article} does not exists"
      exit 1
    fi
  fi

  genrefs "$references_file" >> "$site_article"

  echo "DONE: $article_name"
done
