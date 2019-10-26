This directory contains character reference names
that are supported by HTML, and the code points
to which they refer.

The `entities.json` file was retrieved from
https://html.spec.whatwg.org/entities.json
using ./retrieve.sh on 2019-10-27, and correspond to
*HTML Living Standard — Last Updated 25 October 2019*.

A human readable version of said data is available at
https://html.spec.whatwg.org/multipage/named-characters.html

The data file is Copyright © 2018 WHATWG (Apple, Google, Mozilla, Microsoft).
Their work is licensed under a [Creative Commons Attribution 4.0 International License](https://creativecommons.org/licenses/by/4.0/).

# Updating these files

Run `./retrieve.sh` - this will download the latest JSON
definitions of the character reference names from the
official WHATWG website.
