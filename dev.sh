# Monitor all files in a directory, execute script once when something changes
# if on mac run fswatch -o -r ./src | xargs -n1 -I{} ./build.sh
if [ "$OSTYPE" == "darwin"* ]; then
    fswatch -o -r ./src | xargs -n1 -I{} ./build.sh
fi

# if on linux run while inotifywait -e close_write myfile.py; do ./build.sh; done
if [ "$OSTYPE" == "linux-gnu" ]; then
    while inotifywait -e close_write ./src; do ./build.sh; done
fi