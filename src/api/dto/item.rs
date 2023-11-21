use crate::domain::models::items::{
    GetItemsQuery, GetItemsSortBy, Item, NamesGetItemsQuery, PriceGetItemsQuery, Size,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Default)]
pub struct PriceGetItemsQueryDTO {
    pub from: f64,
    pub to: f64,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct NamesGetItemsQueryDTO {
    pub full: Option<Vec<String>>,
    pub partly: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GetItemsQueryDTO {
    pub ids: Option<Vec<i64>>,
    pub price: Option<PriceGetItemsQueryDTO>,
    pub names: Option<NamesGetItemsQueryDTO>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GetItemsRequestDTO {
    pub offset: i64,
    pub limit: i64,
    pub query: Option<GetItemsQueryDTO>,
    pub sort_by: Option<GetItemsSortByDTO>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GetItemsSortByDTO {
    pub field: String,
    pub desc: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct SizeDTO {
    pub count: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ItemDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub sizes: Vec<SizeDTO>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct CreateItemResponseDTO {
    pub id: i64,
}

impl Into<GetItemsQuery> for GetItemsQueryDTO {
    fn into(self) -> GetItemsQuery {
        let mut query = GetItemsQuery {
            ..Default::default()
        };

        query.ids = self.ids;

        if let Some(price) = self.price {
            query.price = Some(PriceGetItemsQuery {
                from: Some(price.from),
                to: Some(price.to),
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

impl Into<GetItemsSortBy> for GetItemsSortByDTO {
    fn into(self) -> GetItemsSortBy {
        GetItemsSortBy {
            field: self.field,
            desc: self.desc,
        }
    }
}

impl Into<Item> for ItemDTO {
    fn into(self) -> Item {
        Item {
            id: self.id.unwrap_or(0),
            name: self.name,
            description: self.description,
            price: self.price,
            sizes: self
                .sizes
                .into_iter()
                .map(|size| {
                    (
                        Size {
                            id: 0,
                            name: size.name,
                        },
                        size.count,
                    )
                })
                .collect(),
        }
    }
}

impl From<Item> for ItemDTO {
    fn from(item: Item) -> ItemDTO {
        ItemDTO {
            id: Some(item.id),
            name: item.name,
            description: item.description,
            price: item.price,
            sizes: item
                .sizes
                .into_iter()
                .map(|(size, count)| SizeDTO {
                    name: size.name,
                    count,
                })
                .collect(),
        }
    }
}
