import { derived, writable, type Writable } from "svelte/store";

export enum UserPage {
    ScanCampusCard = "ScanCampusCard",
    CampusCardNotFound = "CampusCardNotFound",
    TapeLengthSelection = "TapeLengthSelection",
    RegistrationSuccessful = "RegistrationSuccessful",
    RegistrationExists = "RegistrationExists",
}

export const userPage = writable<UserPage>(UserPage.ScanCampusCard);
export const cardID = writable<string>("");
export const cardNickname = writable<string>("");
export const cardBalance = writable<number>(0.0);