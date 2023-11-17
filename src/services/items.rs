use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::items::{GetItemsQuery, GetItemsSortBy, Item};
use crate::domain::repositories::items::{Repository, TodoQueryParams};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::services::order::CoreService;
use crate::infrastructure::models::items::{ItemDiesel, ItemsSizesDiesel, SizeDiesel};

#[derive(Clone)]
pub struct CoreServiceImpl {
    pub repository: Arc<dyn Repository>,
}

impl CoreServiceImpl {
    pub fn new(repository: Arc<dyn Repository>) -> Self {
        CoreServiceImpl { repository }
    }
}

#[async_trait]
impl CoreService for CoreServiceImpl {
    async fn get_items(
        &self,
        query: Option<GetItemsQuery>,
        sort_by: Option<GetItemsSortBy>,
        offset: i64,
        limit: i64,
    ) -> Result<ResultPaging<Item>, CommonError> {
        let items: Vec<Item> = self
            .repository
            .get_items(query, sort_by, offset, limit)
            .await?
            .into_iter()
            .map(
                |item: (ItemDiesel, Vec<SizeDiesel>, Vec<ItemsSizesDiesel>)| -> Item {
                    item.into()
                },
            )
            .collect();

        Ok(ResultPaging {
            offset,
            total: items.len() as i64,
            items,
        })
    }
}

#[cfg(test)]
mod core_service_tests {
    use std::arch::aarch64::veor_s8;
    use std::future::Future;
    use std::ops::Deref;
    use std::pin::Pin;
    use std::process::Output;
    use super::*;
    use mockall::{mock};
    use std::sync::Arc;
    use std::thread;
    use diesel::serialize::IsNull::No;
    use crate::domain::error::RepositoryError;
    use crate::domain::models::items::{NamesGetItemsQuery, Size};
    use crate::domain::repositories::items::MockRepository;

    fn get_test_data() -> Vec<(ItemDiesel, Vec<SizeDiesel>, Vec<ItemsSizesDiesel>)> {
        vec![
            (ItemDiesel { id: 1, name: "Item 1".to_string(), description: "".to_string(), price: 0.0 },
             vec![SizeDiesel { id: 1, name: "S".to_string() }],
             vec![ItemsSizesDiesel { id: 1, item_id: 1, size_id: 1, quantity: 1 }]),
            (ItemDiesel { id: 2, name: "Item 2".to_string(), description: "".to_string(), price: 0.0 },
             vec![SizeDiesel { id: 2, name: "M".to_string() }],
             vec![ItemsSizesDiesel { id: 0, item_id: 2, size_id: 2, quantity: 0 }]),
        ]
    }

    #[test]
    fn get_items_should_return_all_items_when_no_query_provided() {
        let mut mock_repository = MockRepository::new();
        let mock_items = get_test_data();

        let want = vec![
            Item {
                id: 1,
                name: "Item 1".to_string(),
                description: "".to_string(),
                price: 0.0,
                sizes: vec![(Size { id: 1, name: "S".to_string() }, 1)],
            },
            Item {
                id: 2,
                name: "Item 2".to_string(),
                description: "".to_string(),
                price: 0.0,
                sizes: vec![(Size { id: 2, name: "M".to_string() }, 0)],
            }
        ];

        mock_repository.expect_get_items()
            .return_once(move |q: Option<GetItemsQuery>, sort: Option<GetItemsSortBy>, lim: i64, off: i64| -> _ {
                assert!(q.is_none());
                assert!(sort.is_none());
                assert_eq!(lim, 10);
                assert_eq!(off, 0);

                Box::pin(async move {
                    Ok(mock_items)
                })
            });

        let core_service = CoreServiceImpl::new(Arc::new(mock_repository));

        let thread = thread::spawn(move || async move {
            let res = core_service.get_items(None, None, 0, 10).await.unwrap();

            assert_eq!(res.offset, 0);
            assert_eq!(res.total, 2);
            assert!(res.items==want);
        });

        let x = thread.join().unwrap();
    }

    #[test]
    fn get_items_should_filter_by_name() {
        let mut mock_repository = MockRepository::new();
        let mock_items = get_test_data();

        let query = Some(GetItemsQuery{
            ids: None,
            price: None,
            names: Some(NamesGetItemsQuery{ full: Some(vec!["Item 1".to_string()]), partly: None }),
        });
        let sort = None;
        let limit = 0;
        let offset = 0;

        let q_ref = Arc::new(query.clone());
        let s_ref = Arc::new(sort.clone());

        let want_vec = vec![mock_items[0].clone().into()];
        let want_offset = 0;
        let want_total = 1;

        mock_repository.expect_get_items()
            .return_once(move |q: Option<GetItemsQuery>, s: Option<GetItemsSortBy>, lim: i64, off: i64| -> _ {
                assert_eq!(q, q_ref.deref().clone());
                assert_eq!(s, s_ref.deref().clone());
                assert_eq!(lim, limit);
                assert_eq!(off, offset);

                Box::pin(async move {
                    Ok(vec![mock_items[0].clone()])
                })
            });

        let core_service = CoreServiceImpl::new(Arc::new(mock_repository));

        let thread = thread::spawn(move || async move {
            let res = core_service.get_items(query, sort, limit, offset).await.unwrap();

            assert_eq!(res.offset, want_offset);
            assert_eq!(res.total, want_total);
            assert!(res.items==want_vec);
        });

        let x = thread.join().unwrap();
    }

}
