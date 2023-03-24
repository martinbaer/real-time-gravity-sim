./build.sh
cd pkg
# http-server -S -C ../localhost.pem -K ../localhost-key.pem
# http-server -p 443 --ssl --cert ../localhost.pem --key ../localhost-key.pem
# http-server -p 443 --ssl --cert ../localhost.pem --key ../localhost-key.pem --mimeTypes '{"wasm": "application/wasm"}'

# python3 -m http.server 8000
# ngrok http 8080
docker run -p 443:443 -v $(pwd):/usr/share/nginx/html:ro -v $(pwd)/certs:/etc/nginx/certs -d nginx:latest