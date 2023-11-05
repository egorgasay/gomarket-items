use diesel;
use diesel::prelude::*;
use diesel::sql_types::Integer;
use crate::domain::models::order::{Good, NewOrder};
use crate::infrastructure::schema::{orders,orders_goods,goods};

#[derive(Queryable, Insertable)]
#[diesel(table_name = goods)]
#[derive(Clone)]
pub struct GoodDiesel {
    pub id: String,
    pub price: f64,
}

// Factory method for creating a new GoodDiesel from a Good
impl From<Good> for GoodDiesel {
    fn from(t: Good) -> Self {
        GoodDiesel {
            id: t.description,
            price: t.price,
        }
    }
}

#[derive(Insertable,Queryable)]
#[diesel(table_name = orders)]
pub struct OrderDiesel {
    pub id: String,
}

#[derive(Insertable,Queryable)]
#[diesel(table_name = orders_goods)]
pub struct OrdersGoodsDiesel {
    pub order_id: String,
    pub good_id: String,
}

// Factory method for creating a new Good from a GoodDiesel
impl Into<Good> for GoodDiesel {
    fn into(self) -> Good {
        Good {
            description: self.id,
            price: self.price,
        }
    }
}

impl From<NewOrder> for OrderDiesel {
    fn from(t: NewOrder) -> Self {
        OrderDiesel {
            id: t.order
        }
    }
}

pub fn split_new_order(t: NewOrder) -> (OrderDiesel, Vec<GoodDiesel>, Vec<OrdersGoodsDiesel>) {
    return (OrderDiesel{id: t.order.clone()},
            t.goods.clone()
                .into_iter()
                .map(|g| g.into() )
                .collect(),
            t.goods
                .into_iter()
                .map(|g| OrdersGoodsDiesel {
                    order_id: t.order.clone(),
                    good_id: g.description,
                }).collect(),
    );
}