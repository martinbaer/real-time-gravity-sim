# Space Clicker

Space Clicker is the continuation of my project N-Body Gravity Simulation. It is a browser gravity sim written in Rust and compiled into WebAssembly. This was my first Rust project so I took this as a learning opportunity by re-writing all of the C++ code from this original project.

It is currently a work-in-progress.

## Build dependencies

* Rust
	* Unix-based:<br>
	```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
	* MacOS:<br>
	```brew install rustup```
	* Windows:<br>
	https://forge.rust-lang.org/infra/other-installation-methods.html
* Rust Web-Assembly compiler: wasm-pack and wasm-bindgen-cli
	* ```cargo install wasm-pack```
	* ```cargo install wasm-bindgen-cli```

## Development dependencies

* File system watcher: fswatch
	* MacOS:<br>
	```brew install fswatch```
	* Linux:<br>
	It should be available as ```fswatch``` in your package manager. e.g. ```apt-get install fswatch```
* Any http server to serve static files
	* For similicity I just use VSCode "Open with Live Server" for development