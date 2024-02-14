import { browser } from "$app/environment";
import { page } from "$app/stores";
import { cardBalance, cardID, cardNickname, userPage } from "./stores";

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
		// Actual packet?! crazy
		let data = JSON.parse(event.data);
		if ("user_page" in data) {
			userPage.set(data.user_page);
			cardID.set(data.card_id);
			cardNickname.set(data.card_nickname);
			cardBalance.set(data.card_balance);	
		}
		console.log(data);
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

