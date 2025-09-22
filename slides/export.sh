#!/usr/bin/env bash

echo "Export to which format : "
printf "1. PDF \n2. HTML \n"
read -rp "Enter your option: " option

case "$option" in
1)
  presenterm --export-pdf $1 -x -o PDF/$1.pdf
  ;;
2)
  presenterm --export-html $1 -x -o HTML/$1.html
  ;;
*)
  echo "Invalid Option" >&2
  exit 1
  ;;
esac
