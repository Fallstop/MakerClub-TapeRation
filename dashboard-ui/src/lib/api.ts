import { browser } from "$app/environment";
import { goto } from "$app/navigation";
import { page } from "$app/stores";
import { get, writable } from "svelte/store";
import { adminToken } from "./stores";

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

export function login(password: string | null = null) {
	password = password || get(adminToken);

	// fetch(`http://${location.hostname}:8081/login`, {
	// 	method: 'POST',
	// 	headers: {
	// 		'Content-Type': 'application/json'
	// 	},
	// 	body: JSON.stringify({ password })
	// })
	// .then(response => {
	// 	if (response.ok) {
	// 		console.log('Login successful');
	// 		adminToken.set(password);
	// 		goto('/');
	// 	} else {
	// 		console.error('Login failed');
	//		goto('/login');
	// 	}
	// })
	// .catch(error => {
	// 	console.error('Login error:', error);
	// });

	if(password === 'password') {
		console.log('wow login success incredible');
		adminToken.set(password);
		goto('/');
	} else {
		console.log('fail')
		goto('/login');
	}
}

