rm -rf ./pkg
wasm-pack build
# copy all assets in ./assets/* to ./pkg
cp -r ./assets/* ./pkg
