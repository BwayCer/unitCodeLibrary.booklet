#!/bin/sh

printf "你好, %s!\n" "`
  [ -n "$NAME" ] && echo "$NAME" ||
  [ -n "$1" ] && echo "$1" || echo "世界"
`"

