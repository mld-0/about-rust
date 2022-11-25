#!/usr/bin/env zsh
#set -o errexit   # abort on nonzero exitstatus
set -o nounset   # abort on unbound variable
set -o pipefail  # don't hide errors within pipes

re_date="[0-9]\{4\}-[0-9]\{2\}-[0-9]\{2\}"
grep -r --include "*.rs" "Ongoing: $re_date" | sort

