# Space Clicker

Space Clicker is the continuation of my project N-Body Gravity Simulation. It is a browser gravity sim written in Rust and compiled into WebAssembly. This was my first Rust project so I took this as a learning opportunity by re-writing all of the C++ code from this original project.

It is currently a work-in-progress.

## Build dependencies

* Rust compiler: cargo
	* Unix-based:<br>
	```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
	* MacOS:<br>
	```brew install rustup```
	* Windows:<br>
	https://forge.rust-lang.org/infra/other-installation-methods.html
* Rust Web-Assembly compiler: wasm-pack
	* ```cargo install wasm-pack```
	* ```cargo install wasm-bindgen-cli```

## Development dependencies

* File system watcher: fswatch
	* MacOS:<br>
	```brew install fswatch```
	* Linux:<br>
	It should be available as ```fswatch``` in your package manager. e.g. ```apt-get install fswatch```
* Any http server
	* NPM http server <br>
	Install: ```npm install -g http-server``` <br>
	Run: ```cd ./web && http-server```
	* Python <br>
	```python3 -m http.server 8000```
	* VSCode live development server <br>
	Right click on ./web/index.html > Open with Live Server