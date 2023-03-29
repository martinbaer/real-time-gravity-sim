var canvas = document.getElementById("canvas");
var ctx = canvas.getContext("2d");

function draw_body(x, y, color, size) {
	ctx.fillStyle = color;
	ctx.fillRect(x, y, size, size);
}