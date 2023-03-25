# Monitor all files in a directory, execute script once when something changes
fswatch -o -r ./src | xargs -n1 -I{} ./build.sh