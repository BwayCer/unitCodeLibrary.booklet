#!/bin/sh

printf "Hello, %s!\n" "`
  [ -n "$NAME" ] && echo "$NAME" ||
  [ -n "$1" ] && echo "$1" || echo "world"
`"

