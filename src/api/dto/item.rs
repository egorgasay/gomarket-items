use crate::domain::models::item::{GetItemsQuery, Item, NamesGetItemsQuery, PriceGetItemsQuery};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Copy, Clone, Default)]
pub struct PriceGetItemsQueryDTO {
    pub from: f64,
    pub to: f64,
}

#[derive(Deserialize, Clone, Default)]
pub struct NamesGetItemsQueryDTO {
    pub full: Option<Vec<String>>,
    pub partly: Option<Vec<String>>,
}

#[derive(Deserialize, Clone, Default)]
pub struct GetItemsQueryDTO {
    pub ids: Option<Vec<i64>>,
    pub price: Option<PriceGetItemsQueryDTO>,
    pub names: Option<NamesGetItemsQueryDTO>,
}

#[derive(Deserialize, Clone)]
pub struct GetItemsRequestDTO {
    pub offset: i64,
    pub limit: i64,
    pub query: Option<GetItemsQueryDTO>,
    pub sort_by: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct Size {
    pub count: i32,
    pub name: String,
}

#[derive(Serialize, Clone)]
pub struct ItemDTO {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub sizes: Vec<Size>,
}

impl Into<GetItemsQuery> for GetItemsQueryDTO {
    fn into(self) -> GetItemsQuery {
        GetItemsQuery {
            ids: self.ids.unwrap_or(vec![]),
            price: PriceGetItemsQuery {
                from: self.price.unwrap_or(PriceGetItemsQueryDTO{ from: 0.0, to: 0.0}).from,
                to: self.price.unwrap_or(PriceGetItemsQueryDTO{ from: 0.0, to: 0.0}).from,
            },
            names: NamesGetItemsQuery {
                full: self.names.clone().unwrap_or(Default::default()).full,
                partly: self.names.unwrap_or(Default::default()).partly,
            },
        }
    }
}

impl From<Item> for ItemDTO {
    fn from(item: Item) -> ItemDTO {
        ItemDTO {
            id: item.id,
            name: item.name,
            description: item.description,
            price: item.price,
            sizes: item
                .sizes
                .into_iter()
                .map(|(size, count)| Size {
                    name: size.name,
                    count,
                })
                .collect(),
        }
    }
}
