# Working on chars

This is only a rudimentary document on some common tasks that need to
be done from time to time, but I hope to eventually expand it to
include more topics (:

## Rebuilding search trees and indexes

The top level directory has a Makefile which automates the building of
all the static data that gets compiled into `chars`. To run these
tasks, use `make names`.

## Updating Unicode data

Unicode release new standards from time to time, to better improve
human communication and to allow people to express themselves more
creatively using new emoji. To fetch the latest data files and
integrate them in `chars`, run `make fetch`, followed by `make names`.
