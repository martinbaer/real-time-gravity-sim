import init, { create_bodies, render_bodies } from './space_clicker.js';

const NUM_STARTING_BODIES = 1000;

async function run() {
	await init();
	// From here on we use the functionality defined in wasm.
	canvas.width = window.innerWidth;
	canvas.height = window.innerHeight;
	let is_mobile = false;
	if (/Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent)) {
		// true for mobile device
		is_mobile = true;
	}

	create_bodies(canvas.width, canvas.height, NUM_STARTING_BODIES, is_mobile);

	// function loop() {
	// 	ctx.clearRect(0, 0, canvas.width, canvas.height);
	// 	render_bodies();
	// 	requestAnimationFrame(loop);
	// }
	// loop();
	// same as above by prints out the rate of the loop every second
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


