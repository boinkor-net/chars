#!/bin/bash

set -eu -o pipefail

BASE="$(cd "$(dirname "$0")"; pwd)"

curl https://html.spec.whatwg.org/entities.json -o "${BASE}/entities.json"
