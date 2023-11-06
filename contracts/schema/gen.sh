#!/bin/env sh
# -*- coding: utf-8 -*-

DIR=$(realpath $(dirname "$0"))
NAME=${1/_/ }
NAME=${NAME^}

$DIR/../../deps/yq/bin/yq ".title = \"$NAME Schema\"" template.schema.yml > $DIR/$1.schema.yml
