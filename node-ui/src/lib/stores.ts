import { derived, writable, type Writable } from "svelte/store";

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum UserPage {
//     ScanCampusCard,
//     CampusCardNotFound,
//     ReleaseTape,
//     RegistrationSuccessful,
//     RegistrationExists,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct UIStateUpdate {
//     pub user_page: UserPage,
//     pub card_nickname: Option<String>,
//     pub card_id: Option<String>,
//     pub card_balance: Option<f32>,
// }


export enum UserPage {
    ScanCampusCard = "ScanCampusCard",
    CampusCardNotFound = "CampusCardNotFound",
    ReleaseTape = "ReleaseTape",
    RegistrationSuccessful = "RegistrationSuccessful",
    RegistrationExists = "RegistrationExists",
}

export const userPage = writable<UserPage>(UserPage.ScanCampusCard);
export const cardID = writable<string>("");
export const cardNickname = writable<string>("");
export const cardBalance = writable<number>(0.0);