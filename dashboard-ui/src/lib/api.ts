import { browser } from "$app/environment";
import { page } from "$app/stores";

let socket: WebSocket | null = null;

export async function serverConnect() {
	if (socket) {
		socket.close();
	}
	console.log(location.hostname);
	socket = new WebSocket(`ws://${location.hostname}:8081/stream`);
	socket.onopen = (event) => {
		console.log("Connected to server!");
	};
	socket.onmessage = (event) => {
		// Actual packet?!
		let data = JSON.parse(event.data);
	};
	socket.onclose = (event) => {
		console.error("Websocket Error", event);
		setTimeout(serverConnect, 500);
	};
}

export function sendMessage(message: object) {
	if (socket) {
		socket.send(JSON.stringify(message));
	}
}

