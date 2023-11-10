use crate::domain::models::item::{Item, Size};
use crate::infrastructure::schema::{items, items_sizes, sizes};
use diesel;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[diesel(table_name = items)]
#[derive(Clone)]
pub struct ItemDiesel {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub price: f64,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = sizes)]
#[derive(Clone)]
pub struct SizeDiesel {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = items_sizes)]
#[derive(Clone)]
pub struct ItemsSizesDiesel {
    pub item_id: i64,
    pub size_id: i32,
    pub quantity: i32,
}

fn split_item_to_diesel(t: Item) -> (ItemDiesel, Vec<ItemsSizesDiesel>) {
    (
        ItemDiesel {
            id: t.id,
            name: t.name,
            description: t.description,
            price: t.price,
        },
        t.sizes
            .iter()
            .map(|s| ItemsSizesDiesel {
                item_id: t.id,
                size_id: s.0.id,
                quantity: s.1,
            })
            .collect(),
    )
}

impl Into<Item> for (ItemDiesel, Vec<SizeDiesel>, Vec<ItemsSizesDiesel>) {
    fn into(self) -> Item {
        let mut sizes: Vec<(Size, i32)> = vec![];

        for s in self.2 {
            sizes.push((
                Size {
                    id: s.size_id,
                    name: self.1[0].name.clone(),
                },
                s.quantity,
            ));
        }

        Item {
            id: self.0.id,
            name: self.0.name,
            description: self.0.description,
            price: self.0.price,
            sizes,
        }
    }
}
