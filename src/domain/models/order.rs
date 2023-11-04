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
