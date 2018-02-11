#!/bin/bash

set -eu -o pipefail

BASE="$(cd "$(dirname "$0")"; pwd)"

curl ftp://ftp.unicode.org/Public/UNIDATA/NameAliases.txt -o "${BASE}/NameAliases.txt"
curl ftp://ftp.unicode.org/Public/UNIDATA/UnicodeData.txt -o "${BASE}/UnicodeData.txt"
