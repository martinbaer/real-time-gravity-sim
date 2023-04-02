var canvas = document.getElementById("canvas");
var ctx = canvas.getContext("2d");

function draw_body(x, y, color, size) {
	ctx.fillStyle = color;
	ctx.fillRect(x, y, size, size);
}

function draw_arrow(x1, y1, x2, y2, color) {
	// set arrow properties
	ctx.strokeStyle = color;
	ctx.fillStyle = color;
	ctx.lineWidth = 2;
	ctx.lineCap = "round";

	// draw line
	ctx.beginPath();
	ctx.moveTo(x1, y1);
	ctx.lineTo(x2, y2);
	ctx.stroke();

	// draw arrowhead
	var angle = Math.atan2(y2 - y1, x2 - x1);
	ctx.beginPath();
	ctx.moveTo(x2, y2);
	ctx.lineTo(x2 - 10 * Math.cos(angle - Math.PI / 6), y2 - 10 * Math.sin(angle - Math.PI / 6));
	ctx.lineTo(x2 - 10 * Math.cos(angle + Math.PI / 6), y2 - 10 * Math.sin(angle + Math.PI / 6));
	ctx.lineTo(x2, y2);
	ctx.fill();
}

function increase_num_bodies(num) {
	current_num = parseInt(document.getElementById("num-bodies").innerHTML);
	document.getElementById("num-bodies").innerHTML = current_num + num;
}