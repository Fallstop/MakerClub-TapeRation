import { browser } from "$app/environment";
import { goto } from "$app/navigation";
import { page } from "$app/stores";
import { get, writable } from "svelte/store";
import { adminToken, type User } from "./stores";

let socket: WebSocket | null = null;

export async function serverConnect() {
  if (socket) {
    socket.close();
  }
  console.log(location.hostname);
  socket = new WebSocket(`ws://${location.hostname}:8081/stream/`);
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

// ===== NOT WEBSOCKET STUFF (INCREDIBLE) =====

export async function login(password: string | null = null) {
  password = password || get(adminToken);

  if (!password || password == null) {
    goto("/login");
    return;
  }

  try {
    const response = await fetch(`http://${location.hostname}:8080/api/login`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        password: password || "",
      },
    });

    if (response.ok) {
      console.log(response);
      adminToken.set(password);
      console.log("Login successful");
      goto("/");
    } else {
      console.error("Login failed");
      goto("/login");
    }
  } catch (error) {
    console.error("Login error:", error);
    goto("/login");
  }
}

export function get_participant(campus_card_id: string) {
  fetch(`http://${location.hostname}:8080/api/participant/${campus_card_id}/`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      auth: get(adminToken) || "",
    },
  })
    .then((response) => {
      if (response.ok) {
        console.log("Participant found");
        return response.json() as Promise<User>;
      } else {
        console.error("Participant not found");
        return null;
      }
    })
    .catch((error) => {
      console.error("Participant error:", error);
    });
}

export function add_tape(campus_card_id: string, tape_cm: number) {
  fetch(`http://${location.hostname}:8080/api/campus_card/${campus_card_id}/add?tape_cm=${tape_cm}`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      password: get(adminToken) || "",
    }
  });
}

export function set_tape(campus_card_id: string, tape_cm: number) {
  fetch(`http://${location.hostname}:8080/api/campus_card/${campus_card_id}/set`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
      password: get(adminToken) || "",
    },
    body: JSON.stringify({ tape_cm }),
  });
}

export function reroll_name(campus_card_id: string) {
  fetch(`http://${location.hostname}:8080/api/campus_card/${campus_card_id}/new_name`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      password: get(adminToken) || "",
    },
  });
}

export async function get_all_participants(): Promise<User[]> {
  let response = await fetch(`http://${location.hostname}:8080/api/campus_card`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      password: get(adminToken) || "",
    },
  });
  // console.log(await response.json())
  return (await response.json()).participants as User[];
}

export function add_global_tape(tape_cm: number) {
  fetch(`http://${location.hostname}:8080/api/campus_card/add?tape_cm=${tape_cm}`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      password: get(adminToken) || "",
    },
    // body: JSON.stringify({ tape_cm }),
  });
}

export function set_global_tape(tape_cm: number) {
  fetch(`http://${location.hostname}:8080/api/campus_card/set/`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      password: get(adminToken) || "",
    },
    body: JSON.stringify({ tape_cm }),
  });
}

export function add_user(campus_card: string) {
  fetch(`http://${location.hostname}:8080/api/campus_card/${campus_card}`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
      password: get(adminToken) || "",
    },
  });
}