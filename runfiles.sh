#!/bin/bash

function rust_run(){
    filepath=$(mktemp /tmp/mainXXXXXX)
    rustc "$1" -o "$filepath"
    "$filepath"
    rm "$filepath"
}


shopt -s globstar

declare -A file_paths

echo "Available files"
rust_files=(day*/**/main.rs)
for index in "${!rust_files[@]}"; do
    echo "$((index+1)). ${rust_files[$index]}"
done


echo -e "\n"

max_index=${#rust_files[@]}

while true; do
    read -p "Which file do you want to execute (1 - $max_index): " file_index_to_execute

    if [[ $file_index_to_execute =~ ^[0-9]+$ ]] && (( file_index_to_execute >= 1 && file_index_to_execute <= max_index )); then
        rust_run "${rust_files[$((file_index_to_execute - 1))]}"
        break
    else
        echo "Invalid input. Please enter a number between 1 and $max_index."
    fi
done


