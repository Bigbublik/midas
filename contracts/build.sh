#!/bin/sh
# -*- coding: utf-8 -*-

DIR=`dirname $0`
DIR=`realpath $DIR`

files=`find $DIR/schema -type f -name "*.schema.yml" -and \
  -not -name "template.schema.yml"`

mkdir -p build

for schema_in in $files; do
  echo "YAML -> JSONS: $schema_in"
  JSON_SCHEMA=$DIR/build/`basename $schema_in .schema.yml`.json
  $DIR/../deps/yq/bin/yq -M -o json $schema_in > "$JSON_SCHEMA"
  mkdir -p "$DIR/../frontend/src/rpc"

  echo "JSONS -> Frontend: $schema_in"
  npx quicktype \
    --src "$JSON_SCHEMA" \
    -o "$DIR/../frontend/src/rpc/$(basename $JSON_SCHEMA .json).ts" \
    -l ts --acronym-style original \
    -s schema

  echo "JSONS -> Backend: $schema_in"
done
