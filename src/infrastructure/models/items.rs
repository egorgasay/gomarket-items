use crate::domain::models::items::{Item, Size};
use crate::infrastructure::schema::{items, items_sizes, sizes};
use diesel;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[diesel(table_name = items)]
#[derive(Clone, PartialEq, Debug)]
pub struct ItemDiesel {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub price: f64,
}

#[derive(Insertable)]
#[diesel(table_name = items)]
#[derive(Clone, PartialEq, Debug)]
pub struct SimpleItemDiesel {
    pub name: String,
    pub description: String,
    pub price: f64,
}


#[derive(Queryable, Insertable)]
#[diesel(table_name = sizes)]
#[derive(Clone, PartialEq, Debug)]
pub struct SizeDiesel {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = sizes)]
#[derive(Clone, PartialEq, Debug)]
pub struct SimpleSizeDiesel {
    pub name: String,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = items_sizes)]
#[derive(Clone, PartialEq, Debug)]
pub struct ItemsSizesDiesel {
    pub id: i64,
    pub item_id: i64,
    pub size_id: i32,
    pub quantity: i32,
}

#[derive(Insertable)]
#[diesel(table_name = items_sizes)]
#[derive(Clone, PartialEq, Debug)]
pub struct SimpleItemsSizesDiesel {
    pub item_id: i64,
    pub size_id: i32,
    pub quantity: i32,
}

impl From<(ItemDiesel, Vec<SizeDiesel>, Vec<ItemsSizesDiesel>)> for Item {
    fn from(value: (ItemDiesel, Vec<SizeDiesel>, Vec<ItemsSizesDiesel>)) -> Self {
        let mut sizes: Vec<(Size, i32)> = vec![];

        let find_size_name = Box::new(|isd: &ItemsSizesDiesel| {
            for s in &value.1 {
                if s.id == isd.size_id {
                    return s.name.clone();
                }
            }
            "".to_string()
        });

        for s in &value.2 {
            sizes.push((
                Size {
                    id: s.size_id,
                    name: find_size_name(s),
                },
                s.quantity,
            ));
        }

        Item {
            id: value.0.id,
            name: value.0.name.clone(),
            description: value.0.description.clone(),
            price: value.0.price,
            sizes,
        }
    }
}