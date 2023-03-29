# Monitor all files in a directory, execute script once when something changes
# if on mac run fswatch -o -r ./src | xargs -n1 -I{} ./build.sh
echo "OSTYPE: $OSTYPE"
if [ "$OSTYPE" == "darwin20" ]; then
	echo "Running on mac"
    fswatch -o -r ./src/* | xargs -n1 -I{} ./build.sh
fi

# if on linux run while inotifywait -e close_write myfile.py; do ./build.sh; done
if [ "$OSTYPE" == "linux-gnu" ]; then
	echo "Running on linux"
    while inotifywait -e close_write ./src/*; do ./build.sh; done
fi