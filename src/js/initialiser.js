import init, { create_bodies, render_bodies, on_click, off_click, set_dt, set_gravity, set_spawn_radius, set_spawn_speed, update_mouse_position, set_scale_multiplier } from './space_clicker.js';

const NUM_STARTING_BODIES = 10;


const INITIAL_TIME_STEP = 1;
const TIME_STEP_MULTIPLIER = 0.1;

const INITIAL_GRAVITY = 1;

const INITIAL_SPAWN_RADIUS = 50;

const INITIAL_SPAWN_SPEED = 1;
const SPAWN_SPEED_MULTIPLIER = 0.01;

const BUTTON_SCALE_FACTOR = 1.5;

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

	// VARIABLES
	set_dt(INITIAL_TIME_STEP * TIME_STEP_MULTIPLIER);
	document.getElementById("time-step").innerHTML = INITIAL_TIME_STEP;
	set_gravity(INITIAL_GRAVITY);
	document.getElementById("gravity").innerHTML = INITIAL_GRAVITY;
	set_spawn_radius(INITIAL_SPAWN_RADIUS);
	document.getElementById("spawn-radius").innerHTML = INITIAL_SPAWN_RADIUS;
	set_spawn_speed(INITIAL_SPAWN_SPEED * SPAWN_SPEED_MULTIPLIER);
	document.getElementById("spawn-speed").innerHTML = INITIAL_SPAWN_SPEED;


	// Add listeners for the parameters
	document.getElementById("time-step").addEventListener("blur", function (e) {
		let timeStep = cleanNumberInput(e.target.innerHTML);
		e.target.innerHTML = timeStep;
		set_dt(timeStep * TIME_STEP_MULTIPLIER);

	});
	document.getElementById("gravity").addEventListener("blur", function (e) {
		let gravity = cleanNumberInput(e.target.innerHTML);
		e.target.innerHTML = gravity;
		set_gravity(gravity);
	});
	document.getElementById("spawn-radius").addEventListener("blur", function (e) {
		let spawnRadius = cleanNumberInput(e.target.innerHTML);
		e.target.innerHTML = spawnRadius;
		set_spawn_radius(spawnRadius);
	});
	document.getElementById("spawn-speed").addEventListener("blur", function (e) {
		let spawnSpeed = cleanNumberInput(e.target.innerHTML);
		e.target.innerHTML = spawnSpeed;
		set_spawn_speed(spawnSpeed * SPAWN_SPEED_MULTIPLIER);
	});
	document.getElementById("increase-time-step").addEventListener("click", function (e) {
		let timeStep = cleanNumberInput(document.getElementById("time-step").innerHTML);
		timeStep *= BUTTON_SCALE_FACTOR;
		document.getElementById("time-step").innerHTML = timeStep;
		set_dt(timeStep * TIME_STEP_MULTIPLIER);
	});
	document.getElementById("decrease-time-step").addEventListener("click", function (e) {
		let timeStep = cleanNumberInput(document.getElementById("time-step").innerHTML);
		timeStep /= BUTTON_SCALE_FACTOR;
		document.getElementById("time-step").innerHTML = timeStep;
		set_dt(timeStep * TIME_STEP_MULTIPLIER);
	});
	document.getElementById("increase-gravity").addEventListener("click", function (e) {
		let gravity = cleanNumberInput(document.getElementById("gravity").innerHTML);
		gravity *= BUTTON_SCALE_FACTOR;
		document.getElementById("gravity").innerHTML = gravity;
		set_gravity(gravity);
	});
	document.getElementById("decrease-gravity").addEventListener("click", function (e) {
		let gravity = cleanNumberInput(document.getElementById("gravity").innerHTML);
		gravity /= BUTTON_SCALE_FACTOR;
		document.getElementById("gravity").innerHTML = gravity;
		set_gravity(gravity);
	});
	document.getElementById("increase-spawn-radius").addEventListener("click", function (e) {
		let spawnRadius = cleanNumberInput(document.getElementById("spawn-radius").innerHTML);
		spawnRadius *= BUTTON_SCALE_FACTOR;
		document.getElementById("spawn-radius").innerHTML = spawnRadius;
		set_spawn_radius(spawnRadius);
	});
	document.getElementById("decrease-spawn-radius").addEventListener("click", function (e) {
		let spawnRadius = cleanNumberInput(document.getElementById("spawn-radius").innerHTML);
		spawnRadius /= BUTTON_SCALE_FACTOR;
		document.getElementById("spawn-radius").innerHTML = spawnRadius;
		set_spawn_radius(spawnRadius);
	});
	document.getElementById("increase-spawn-speed").addEventListener("click", function (e) {
		let spawnSpeed = cleanNumberInput(document.getElementById("spawn-speed").innerHTML);
		spawnSpeed *= BUTTON_SCALE_FACTOR;
		document.getElementById("spawn-speed").innerHTML = spawnSpeed;
		set_spawn_speed(spawnSpeed * SPAWN_SPEED_MULTIPLIER);
	});
	document.getElementById("decrease-spawn-speed").addEventListener("click", function (e) {
		let spawnSpeed = cleanNumberInput(document.getElementById("spawn-speed").innerHTML);
		spawnSpeed /= BUTTON_SCALE_FACTOR;
		document.getElementById("spawn-speed").innerHTML = spawnSpeed;
		set_spawn_speed(spawnSpeed * SPAWN_SPEED_MULTIPLIER);
	});

	// document.getElementById("gravity").addEventListener("blur", function (e) {
	// 	let gravity = cleanNumberInput(e.target.innerHTML);
	// 	e.target.innerHTML = gravity;
	// 	set_gravity(gravity);
	// });

	document.getElementById("zoom").addEventListener("blur", function (e) {
		let zoom = cleanNumberInput(e.target.innerHTML);
		e.target.innerHTML = zoom;
		set_scale_multiplier(zoom);
	});
	document.getElementById("increase-zoom").addEventListener("click", function (e) {
		let zoom = cleanNumberInput(document.getElementById("zoom").innerHTML);
		zoom *= BUTTON_SCALE_FACTOR;
		document.getElementById("zoom").innerHTML = zoom;
		set_scale_multiplier(zoom);
	});
	document.getElementById("decrease-zoom").addEventListener("click", function (e) {
		let zoom = cleanNumberInput(document.getElementById("zoom").innerHTML);
		zoom /= BUTTON_SCALE_FACTOR;
		document.getElementById("zoom").innerHTML = zoom;
		set_scale_multiplier(zoom);
	});

	// Add spawning listeners
	canvas.addEventListener("mousedown", function (e) {
		on_click(e.clientX, e.clientY);
		update_mouse_position(e.clientX, e.clientY);
	});
	canvas.addEventListener("mouseup", function (e) {
		off_click(e.clientX, e.clientY);
	});
	let body_spawning_active = false;
	canvas.addEventListener("touchstart", function (e) {
		e.preventDefault();
		body_spawning_active = !body_spawning_active;
		if (body_spawning_active) {
			on_click(e.touches[0].clientX, e.touches[0].clientY);
		} else {
			off_click(e.touches[0].clientX, e.touches[0].clientY);
		}
	});


	// Add mouse move listener
	document.addEventListener("mousemove", function (event) {
		update_mouse_position(event.clientX, event.clientY);
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

// removes non numbers (excluding ".") and if the string is empty returns "0"
function cleanNumberInput(string) {
	let cleanString = string.replace(/[^0-9.]/g, "");
	if (cleanString === "") {
		return "0";
	}
	return cleanString;
}