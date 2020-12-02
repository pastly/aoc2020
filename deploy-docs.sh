#!/usr/bin/env bash
set -eu
cargo doc
rsync -air --delete target/doc/ piggie:/var/www/demos.traudt.xyz/aoc2020
