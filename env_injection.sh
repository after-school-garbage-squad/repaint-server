#!/bin/bash

tmpfile="/tmp/tempfile_$RANDOM"

cat $1 > $tmpfile

if [ -f .env ]; then
  while read -r line || [[ -n "$line" ]]; do
    key="${line%%=*}"
    value="${line#*=}"

    value="${value#"${value%%[![:space:]]*}"}"
    value="${value%"${value##*[![:space:]]}"}"
    
    sed -i -e s/{{$key}}/$value/g $tmpfile
  done < .env
else
  echo "エラー: .envファイルが見つかりません"
  exit 1
fi

cat "$tmpfile"

rm "$tmpfile"
