#!/bin/sh

tmpfile="/tmp/tempfile_$RANDOM"

cat $1 > $tmpfile

for env_var in $(printenv); do
  key=$(echo "$env_var" | cut -d '=' -f1)
  value=$(echo "$env_var" | cut -d '=' -f2-)

  value="${value#"${value%%[![:space:]]*}"}"
  value="${value%"${value##*[![:space:]]}"}"
  value=$(echo "$value" | sed 's/\//\\\//g')

  sed -i -e s/{{$key}}/$value/g $tmpfile
done

if [ $# -ge 2 ]; then
  mv "$tmpfile" "$2"
else
  cat "$tmpfile"
  rm "$tmpfile"
fi
