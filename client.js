const { WebSocket } = require("ws");

const client = new WebSocket("ws://localhost:8080/random-u64");

client.on("error", console.error);

client.on("open", function open() {
	client.send("something");
});

client.on("message", function message(data) {
	console.log("received: %s", data);
});
