rm -rf ./pkg
wasm-pack build --target web
# copy all assets in ./assets/* to ./pkg
cp -r ./assets/* ./pkg
# copy index.html and index.js to ./pkg
cp ./src/index.html ./pkg
# convert app.ts to app.js and move to ./pkg
tsc ./src/app.ts --outFile ./pkg/app.js