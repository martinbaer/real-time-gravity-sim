import init, { create_bodies, render_bodies } from './space_clicker.js';

const NUM_STARTING_BODIES = 3000;

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

	function loop() {
		ctx.clearRect(0, 0, canvas.width, canvas.height);
		render_bodies();
		requestAnimationFrame(loop);
	}
	loop();
	// will the above loop be a constant speed
	// or will it be affected by the speed of the computer?
	// a: it will be affected by the speed of the computer
	// how do we make it a constant speed?

}

run();


