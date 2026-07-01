#!/bin/bash

CONFIG_FILE="$HOME/.kb_list"
ACCESS_DIR=".access"

[ ! -d "$ACCESS_DIR" ] && mkdir -p "$ACCESS_DIR"

toggle_kb() {
    local target_dir="$1"
    local dir_name=$(basename "$target_dir")
    local link_path="$ACCESS_DIR/$dir_name"

    # Check if the symlink exists AND points to the target
    if [ -L "$link_path" ]; then
        # If it exists, remove it
        rm "$link_path"
        echo "Detached $dir_name"
    else
        # If it doesn't exist, create it
        ln -s "$target_dir" "$link_path"
        echo "Attached $dir_name"
    fi
}

# If an argument is provided, toggle that specific one
if [ -n "$1" ]; then
    toggle_kb "$1"
else
    # Otherwise, show a menu of available modules from ~/.kb_list
    echo "Select KB module to toggle in $ACCESS_DIR:"
    select dir in $(cat "$CONFIG_FILE"); do
        if [ -z "$dir" ]; then exit 0; fi
        toggle_kb "$dir"
        break
    done
fi
