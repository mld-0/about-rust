#!/usr/bin/env zsh
#	{{{3
#   vim: set tabstop=4 modeline modelines=10:
#   vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
#	{{{2
set -o errexit   # abort on nonzero exitstatus
set -o nounset   # abort on unbound variable
set -o pipefail  # don't hide errors within pipes

SCRIPTPATH="$( cd "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
dir_items="sourcefiles"

log_debug() {
	echo "$@" > /dev/stderr
}

get_item_paths() {
	paths=( [0-9]*/src/main.rs )
	echo "${paths[@]}" | sort -h
}

link_items() {
	#	{{{
	local func_name=""
	if [[ -n "${ZSH_VERSION:-}" ]]; then 
		func_name=${funcstack[1]:-}
	elif [[ -n "${BASH_VERSION:-}" ]]; then
		func_name="${FUNCNAME[0]:-}"
	else
		printf "%s\n" "warning, func_name unset, non zsh/bash shell" > /dev/stderr
	fi
	#	}}}
	local func_about="about"
	local func_help="""$func_name, $func_about
	-h | --help"""
	#	{{{
	if echo "${1:-}" | perl -wne '/^\s*-h|--help\s*$/ or exit 1'; then
		echo "$func_help"
		return 2
	fi
	#	}}}

	cd "$SCRIPTPATH"
	if [[ -d "$dir_items" ]]; then
		#log_debug $func_name, rm -r "$dir_items"
		#rm -r "$dir_items"
		echo "$func_name, error, already exists, dir_items=($dir_items)"
		exit 2
	fi
	log_debug $func_name, mkdir "$dir_items"
	mkdir "$dir_items"

	paths=( $( get_item_paths ) )
	for f in "${paths[@]}"; do
		log_debug "$func_name, f=($f)"
		f_dest="$dir_items/$( echo "$f" | cut -d "/" -f1 ).rs"
		log_debug "$func_name, f_dest=($f_dest)"
		log_debug $func_name, ln -s "../$f" "$f_dest"
		ln -s "../$f" "$f_dest"
	done
}

link_items

