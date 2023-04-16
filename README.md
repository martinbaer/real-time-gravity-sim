# Real Time Gravity Sim

A simple, interactive, web-based gravity visualiser. 

Live demo: https://martinbaer.dev/real-time-gravity-sim

"Real Time Gravity Sim" is a continuation of my project "Batch Processed Gravity Sim". I started this project as a way of learning how JavaScript and WebAssembly interact. For this version, I compile WebAssemble from Rust instead of C++ to minimise runtime errors in development.

Simulation rules:
* The universe is infinite and begins empty.
* Bodies can be created.
* Bodies are infinitely small points with equal mass.
* The only force acting on bodies is gravitational attraction.

Design:
* Real-time.
* The goal was to be easily interactive - just click-and-drag to create matter.
* Minimalist GUI for adjustable parameters
* Has an auto-centred view that follows centre of mass and scales with simulation

Implementation:
* Accleration solver: Barnes-Hut approximation - O(n log n)
* Made for web using WebAssembly and JavaScript for portability
* The system state and algorithm are written in Rust compiled to WebAssembly
* JavaScript is used to convey HTML canvas API calls and event listener callbacks
* Serial (for now)

## How to Use

* Click and hold to create mass. 
* Drag to give the mass velocity. 
* You can adjust the "zoom", mass creation settings, and physical parameters.
* The camera will move to show most mass automatically.

You can run the simulation by opening the live demo on any WebAssembly enabled device at https://martinbaer.dev/real-time-gravity-sim.

Alternatively, you can locally compile and run the program by 
1. Installing the [build dependencies](#build-dependencies)
2. Pulling this repository.
3. Running ```build.sh``` on Linux or MacOS
4. Hosting the files in the web directory with any server. E.g. ```python -m http.server --directory ./web 8080``` to view in your browser at http://localhost:8080. This is because just opening index.html in your browser (file protocol), will make CORS block WebAssembly.

## Build Dependencies

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
