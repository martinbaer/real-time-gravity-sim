# Real Time Gravity Sim

A simple, interactive, web-based gravity visualiser. 

Live demo: https://martinbaer.dev/real-time-gravity-sim

"Real Time Gravity Sim" is a continuation of my project "Batch Processed Gravity Sim". I started this project as a way of learning how JavaScript and WebAssembly interact. For this version, I compile WebAssemble from Rust instead of C++ to minimise runtime errors in development.

Key UX design choices:
* Real-time
* Easily interactive - click-and-drag to create matter
* Minimalist GUI for adjustable parameters
* Auto-centred view - follows centre of mass and scales with simulation

Key software design choices:
* Implements a Barnes-Hut approximation for O(n log n) time complexity
* Built for web environment using WebAssembly and JavaScript for accessibility and portability
* Near-native performance with system state and algorithm written in Rust compiled to WebAssembly
* JavaScript to convey HTML canvas API calls and event listener callbacks
* Serial (for now)

Key simulation design choices:
* The universe is infinite and begins empty
* Bodies can be created
* Bodies are infinitely small points with equal mass
* The only force acting on bodies is gravitational attraction


## How to Use

## Build dependencies

* Rust
	* Unix-based:<br>
	```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
	* MacOS:<br>
	```brew install rustup```
	* Windows:<br>
	https://forge.rust-lang.org/infra/other-installation-methods.html
* wasm-bindgen-cli
	* ```cargo install wasm-bindgen-cli```

<!-- ## Development dependencies

* File system watcher: fswatch
	* MacOS:<br>
	```brew install fswatch```
	* Linux:<br>
	It should be available as ```fswatch``` in your package manager. e.g. ```apt-get install fswatch```
* Any http server to serve static files
	* For similicity I just use VSCode "Open with Live Server" for development -->