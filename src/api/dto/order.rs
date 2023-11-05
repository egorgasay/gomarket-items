use crate::domain::models::order::{NewOrder, Good, Mechanic, RewardType};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct GoodDTO {
    pub description: String,
    pub price: f64
}

#[derive(Deserialize, Serialize)]
pub struct NewOrderDTO {
    pub order: String,
    pub goods: Vec<GoodDTO>
}

#[derive(Deserialize, Serialize)]
pub struct MechanicDTO {
    #[serde(rename = "match")]
    pub match_word: String,
    pub reward: f64,
    pub reward_type: String
}

impl Into<NewOrder> for NewOrderDTO {
    fn into(self) -> NewOrder {
        NewOrder {
            order: self.order,
            goods: self.goods
                .into_iter()
                .map(|good|
                    good.into()
                )
                .collect(),
        }
    }
}

impl Into<Mechanic> for MechanicDTO {
    fn into(self) -> Mechanic {
        Mechanic {
            match_word: self.match_word,
            reward: self.reward,
            reward_type: {
                match self.reward_type.as_str() {
                    "pt" => RewardType::Fixed,
                    "%" => RewardType::Percent,
                    _ => {
                        panic!("Unknown reward type: {}", self.reward_type)
                    }
                }
            },
        }
    }
}

impl Into<NewOrderDTO> for NewOrder {
    fn into(self) -> NewOrderDTO {
        NewOrderDTO {
            order: self.order,
            goods: self.goods
                .into_iter()
                .map(|good| good.into())
                .collect(),
        }
    }
}

impl Into<GoodDTO> for Good {
    fn into(self) -> GoodDTO {
        GoodDTO {
            description: self.description,
            price: self.price,
        }
    }
}

impl Into<Good> for GoodDTO {
    fn into(self) -> Good {
        Good {
            description: self.description,
            price: self.price,
        }
    }
}