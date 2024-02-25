import { writable, type Writable } from "svelte/store"
import { browser } from "$app/environment";

let LOCAL_STORAGE_VERSION = 1;

/** Makes a store persistent in local storage
 * @param  {[Writable]} store Writable store to be made persistent
 * @param  {[string]} key Key to be used in Local Storage
 */
function useLocalStorage<T>(store: Writable<T>, key: string) {
	let localStorageKey = `tape_v${LOCAL_STORAGE_VERSION}_${key}`;
	if (typeof localStorage !== 'undefined') {
		const json = localStorage.getItem(localStorageKey);
		if (json) {
			store.set(JSON.parse(json));
		}

		store.subscribe((current) => {
			localStorage.setItem(localStorageKey, JSON.stringify(current));
		});
	}
}

export let adminToken: Writable<string | null> = writable(null);
useLocalStorage(adminToken, 'adminToken');

export type User = {
    id: number;
    campus_card: string;
    nick_name: string;
    date_registered: string;
    last_transaction: string;
    tape_left_cm: number;
};