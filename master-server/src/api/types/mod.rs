use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TapeLeft {
    pub tape_left_cm: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Participants {
    pub participants: Vec<crate::db::entities::participants::Model>,
}
