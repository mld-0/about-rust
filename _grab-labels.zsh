#!/usr/bin/env zsh
#set -o errexit   # abort on nonzero exitstatus
set -o nounset   # abort on unbound variable
set -o pipefail  # don't hide errors within pipes

re_date="[0-9]\{4\}-[0-9]\{2\}-[0-9]\{2\}"

files=( `find . -name "*.rs" | sort` )

for f in "${files[@]}"; do
	matches=`grep "Ongoing: $re_date" $f | sed "s|^\s*//\s*||g"`
	if [[ ! -z $matches ]]; then
		echo "$f"
		echo "$matches"
		echo ""
	fi
done

