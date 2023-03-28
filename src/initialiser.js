import init, { create_bodies, render_bodies } from './space_clicker.js';
async function run() {
	await init();
	// From here on we use the functionality defined in wasm.
	canvas.width = window.innerWidth;
	canvas.height = window.innerHeight;
	create_bodies(canvas.width, canvas.height, 1000);

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


