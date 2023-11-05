use diesel;
use diesel::prelude::*;
use crate::domain::models::order::{Mechanic, RewardType};
use crate::infrastructure::schema::{mechanics};


#[derive(Queryable, Insertable)]
#[diesel(table_name = mechanics)]
#[derive(Clone)]
pub struct MechanicDiesel {
    pub match_word: String,
    pub reward: f64,
    pub reward_type: String,
}

impl From<Mechanic> for MechanicDiesel {
    fn from(t: Mechanic) -> Self {
        MechanicDiesel {
            match_word: t.match_word,
            reward: t.reward,
            reward_type: match t.reward_type {
                RewardType::Fixed => "pt".to_string(),
                RewardType::Percent => "%".to_string(),
            },
        }
    }
}