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
        let mut query = GetItemsQuery {
            ..Default::default()
        };

        query.ids = self.ids;

        if let Some(price) = self.price {
            query.price = Some(PriceGetItemsQuery {
                from: price.from,
                to: price.to,
            });
        }

        if let Some(names) = self.names {
            query.names = Some(NamesGetItemsQuery {
                full: names.full,
                partly: names.partly,
            });
        };

        query
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
