use crate::domain::models::order::{NewOrder, Good};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct GoodDTO {
    pub description: String,
    pub price: i32
}

#[derive(Deserialize, Serialize)]
pub struct NewOrderDTO {
    pub order: String,
    pub goods: Vec<GoodDTO>
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