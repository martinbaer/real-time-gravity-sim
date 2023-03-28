rm -rf ./web
cargo build --target wasm32-unknown-unknown # --release
wasm-bindgen target/wasm32-unknown-unknown/debug/space_clicker.wasm --out-dir ./web/ --target web
cp src/index.html ./web/
cp src/initialiser.js ./web/
cp src/draw_body.js ./web/
# copy all assets in ./assets/* to ./web
cp -r ./assets/* ./web


# wasm-pack build --target web
# # copy all assets in ./assets/* to ./pkg
# cp -r ./assets/* ./pkg
# # copy index.html and index.js to ./pkg
# cp ./src/index.html ./pkg
# # copy app.js to ./pkg
# cp ./src/app.js ./pkg