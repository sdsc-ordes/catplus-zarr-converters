#!/usr/bin/env bash
set -eu

binary="$1"
with_strings="${2:-}"

strings=$(strings "$binary")
linking=$(ldd "$binary")

rpaths=$(readelf -d "$binary")
rpaths2=$(objdump -x "$binary")
rpaths3=$(patchelf --print-rpath "$binary")

echo ">>> RPATHS:"
echo "$rpaths3" | tr ':' '\n' | xargs printf "- '%s'\n"

echo ">>> All found /nix/store shit..."
{
	echo "Strings ================"
	[ "$with_strings" = "--with-strings" ] && echo "$strings"

	echo "======================="
	echo "Linking"
	echo "$linking"
	echo "======================="
	echo "RPATHS etc"
	echo "$rpaths"
	echo "$rpaths2"
	echo "$rpaths3"
	echo "======================="
} | grep -o -P '/nix/store/[\w\.-]+\b' | sort -u

if echo "$binary" | grep -q "/nix/store"; then
	echo ">>> Nix reported dependencies"
	nix path-info --recursive "$binary"
fi

read -rp "Do you want to open nix-tree? (yes/no): " answer
case "$answer" in
[Yy][Ee][Ss] | [Yy])
	nix-tree "$binary"
	;;
*) ;;
esac
