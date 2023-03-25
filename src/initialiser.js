import init, { create_particles, render_particles } from './space_clicker.js';
async function run() {
	await init();
	// From here on we use the functionality defined in wasm.
	canvas.width = window.innerWidth;
	canvas.height = window.innerHeight;
	create_particles(canvas.width, canvas.height, 1000);

	function loop() {
		ctx.clearRect(0, 0, canvas.width, canvas.height);
		render_particles();
		requestAnimationFrame(loop);
	}
	loop();
}

run();


