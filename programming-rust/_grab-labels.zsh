#!/usr/bin/env zsh
#set -o errexit   # abort on nonzero exitstatus
set -o nounset   # abort on unbound variable
set -o pipefail  # don't hide errors within pipes
nl=$'\n'

labels=( "Ongoing" "Continue" )
labels=( "Continue" )
re_date="[0-9]\{4\}-[0-9]\{2\}-[0-9]\{2\}"

files=( `find . -name "*.rs" | sort` )

for f in "${files[@]}"; do
	matches=""
	for l in "${labels[@]}"; do
		matches=$matches`grep "$l: $re_date" $f | sed "s|^\s*//\s*||g"`$nl
	done
	matches=`echo "$matches" | sort -h | sed "/^$/d"`
	if [[ ! -z $matches ]]; then
		echo "$f"
		echo "$matches"
		echo ""
	fi
done

