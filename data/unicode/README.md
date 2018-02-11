This directory contains unicode name lists.

The files were retrieved from ftp://ftp.unicode.org/Public/UNIDATA/
using ./retrieve.sh on 2018-02-11, and correspond to Unicode 10.0.

The data files are © 1991-2018 Unicode®, Inc.
For terms of use, see http://www.unicode.org/terms_of_use.html

# Updating these files

Run `./retrieve.sh` - this will download the latest textual
definitions of unicode data from the official FTP site. Once
retrieved, you have to run the generator.

## Format
The format that we recognize is as follows:

```
<hex codepoint>;<name>;[ignored]
```

This allows us to parse both NameList.txt and NameAliases.txt.
