#!/usr/bin/env bash

# See: https://github.com/kyclark/command-line-rust

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

echo    "Hello there"    > $OUTDIR/hello1.txt
echo    "Hello"  "there" > $OUTDIR/hello2.txt
echo -n "Hello  there"   > $OUTDIR/hello1.n.txt
echo -n "Hello"  "there" > $OUTDIR/hello2.n.txt
