use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Good {
    pub description: String,
    pub price: f64
}

#[derive(Clone)]
pub struct NewOrder {
    pub order: String,
    pub goods: Vec<Good>
}

#[derive(Clone)]
pub struct Mechanic {
    pub match_word: String,
    pub reward: f64,
    pub reward_type: RewardType
}

#[derive(Clone)]
pub enum RewardType {
    Fixed,
    Percent,
}

