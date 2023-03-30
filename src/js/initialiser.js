import init, { create_bodies, render_bodies, on_click, off_click } from './space_clicker.js';

const NUM_STARTING_BODIES = 3;

async function run() {
	await init();
	// Gather info needed in the wasm module
	canvas.width = window.innerWidth;
	canvas.height = window.innerHeight;
	let is_mobile = false;
	if (/Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent)) {
		// true for mobile device
		is_mobile = true;
	}
	// Initialise the simulation
	create_bodies(canvas.width, canvas.height, NUM_STARTING_BODIES, is_mobile);
	// Add interaction listeners
	let body_spawning_active = false;
	canvas.addEventListener("mouseup", function (e) {
		body_spawning_active = !body_spawning_active;
		if (body_spawning_active) {
			on_click(e.clientX, e.clientY);
		} else {
			off_click(e.clientX, e.clientY);
		}
		// off_click(e.clientX, e.clientY);
	});
	canvas.addEventListener("touchstart", function (e) {
		e.preventDefault();
		body_spawning_active = !body_spawning_active;
		if (body_spawning_active) {
			on_click(e.touches[0].clientX, e.touches[0].clientY);
		} else {
			off_click(e.touches[0].clientX, e.touches[0].clientY);
		}
	});

	// Start the animation loop
	let last_time = Date.now();
	let frames = 0;
	function loop() {
		ctx.clearRect(0, 0, canvas.width, canvas.height);
		render_bodies();
		frames += 1;
		let now = Date.now();
		let delta = now - last_time;
		if (delta > 1000) {
			console.log("FPS: " + frames);
			frames = 0;
			last_time = now;
		}
		requestAnimationFrame(loop);
	}
	loop();
}

run();


