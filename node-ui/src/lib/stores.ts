import { tweened } from "svelte/motion";
import { derived, writable, type Writable } from "svelte/store";

export enum UserPage {
    ScanCampusCard = "ScanCampusCard",
    CampusCardNotFound = "CampusCardNotFound",
    TapeLengthSelection = "TapeLengthSelection",
    RegistrationSuccessful = "RegistrationSuccessful",
    RegistrationExists = "RegistrationExists",
}

export const defaultUserPage = UserPage.ScanCampusCard;

export const userPage = writable<UserPage>(UserPage.ScanCampusCard);
export const cardID = writable<string>("");
export const cardNickname = writable<string>("");
export const cardBalance = writable<number>(0.0);
export const cardBalanceTweened = tweened(0.0, { duration: 300 });
cardBalance.subscribe((value) => cardBalanceTweened.set(value ?? 0));


export const tapeOptionsCM = writable<number[]>([])