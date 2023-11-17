use std::arch::aarch64::float32x2_t;

#[derive(Clone,PartialEq)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub sizes: Vec<(Size, i32)>,
}

#[derive(Clone, PartialEq)]
pub struct Size {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct PriceGetItemsQuery {
    pub from: f64,
    pub to: f64,
}

#[derive(Clone, PartialEq, Debug)]
pub struct NamesGetItemsQuery {
    pub full: Option<Vec<String>>,
    pub partly: Option<Vec<String>>,
}

#[derive(Clone, Default, PartialEq, Debug)]
pub struct GetItemsQuery {
    pub ids: Option<Vec<i64>>,
    pub price: Option<PriceGetItemsQuery>,
    pub names: Option<NamesGetItemsQuery>,
}

#[derive(Clone, Default, PartialEq, Debug)]
pub struct GetItemsSortBy {
    pub field: String,
    pub desc: bool,
}