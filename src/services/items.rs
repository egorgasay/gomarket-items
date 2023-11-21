use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::items::{GetItemsQuery, GetItemsSortBy, Item};
use crate::domain::repositories::items::Repository;
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
            .map(Item::from)
            .collect();

        Ok(ResultPaging {
            offset,
            total: items.len() as i64,
            items,
        })
    }

    async fn create_item(&self, item: Item) -> Result<i64, CommonError> {
        let item_id = self.repository.create_item(item).await?;

        Ok(item_id)
    }
}

#[cfg(test)]
mod core_service_tests {
    use super::*;
    use crate::domain::models::items::{NamesGetItemsQuery, PriceGetItemsQuery, Size};
    use crate::domain::repositories::items::MockRepository;
    use std::sync::Arc;
    use crate::domain::error::{CommonErrorKind, RepositoryError, RepositoryErrorKind};

    fn get_test_data() -> Vec<(ItemDiesel, Vec<SizeDiesel>, Vec<ItemsSizesDiesel>)> {
        vec![
            (
                ItemDiesel {
                    id: 1,
                    name: "Item 1".to_string(),
                    description: "".to_string(),
                    price: 0.0,
                },
                vec![SizeDiesel {
                    id: 1,
                    name: "S".to_string(),
                }],
                vec![ItemsSizesDiesel {
                    id: 1,
                    item_id: 1,
                    size_id: 1,
                    quantity: 1,
                }],
            ),
            (
                ItemDiesel {
                    id: 2,
                    name: "Item 2".to_string(),
                    description: "".to_string(),
                    price: 0.0,
                },
                vec![SizeDiesel {
                    id: 2,
                    name: "M".to_string(),
                }],
                vec![ItemsSizesDiesel {
                    id: 0,
                    item_id: 2,
                    size_id: 2,
                    quantity: 0,
                }],
            ),
        ]
    }

    #[tokio::test]
    async fn get_items_should_return_all_items_when_no_query_provided() {
        let mut mock_repository = MockRepository::new();
        let mock_items = get_test_data();

        let want = vec![
            Item {
                id: 1,
                name: "Item 1".to_string(),
                description: "".to_string(),
                price: 0.0,
                sizes: vec![(
                    Size {
                        id: 1,
                        name: "S".to_string(),
                    },
                    1,
                )],
            },
            Item {
                id: 2,
                name: "Item 2".to_string(),
                description: "".to_string(),
                price: 0.0,
                sizes: vec![(
                    Size {
                        id: 2,
                        name: "M".to_string(),
                    },
                    0,
                )],
            },
        ];

        mock_repository.expect_get_items().return_once(
            move |q: Option<GetItemsQuery>,
                  sort: Option<GetItemsSortBy>,
                  offset: i64,
                  limit: i64|
                  -> _ {
                assert!(q.is_none());
                assert!(sort.is_none());
                assert_eq!(offset, 0);
                assert_eq!(limit, 10);

                Box::pin(async move { Ok(mock_items) })
            },
        );

        let core_service = CoreServiceImpl::new(Arc::new(mock_repository));
        let res = core_service.get_items(None, None, 0, 10).await.unwrap();

        assert_eq!(res.offset, 0);
        assert_eq!(res.total, 2);
        assert!(res.items == want);
    }

    #[tokio::test]
    async fn get_items_should_filter_by_name() {
        let mut mock_repository = MockRepository::new();
        let mock_items = get_test_data();

        let query = Some(GetItemsQuery {
            ids: None,
            price: None,
            names: Some(NamesGetItemsQuery {
                full: Some(vec!["Item 1".to_string()]),
                partly: None,
            }),
        });
        let sort = None;
        let limit = 0;
        let offset = 0;

        let want_vec = vec![mock_items[0].clone().into()];
        let want_offset = 0;
        let want_total = 1;

        mock_repository.expect_get_items().return_once(
            move |q: Option<GetItemsQuery>, s: Option<GetItemsSortBy>, lim: i64, off: i64| -> _ {
                if let Some(query) = Some(q.clone()) {
                    assert_eq!(q, query);
                }

                if let Some(sort) = Some(s.clone()) {
                    assert_eq!(s, sort);
                }

                assert_eq!(lim, limit);
                assert_eq!(off, offset);

                Box::pin(async move { Ok(vec![mock_items[0].clone()]) })
            },
        );

        let core_service = CoreServiceImpl::new(Arc::new(mock_repository));

        let res = core_service
            .get_items(query, sort, limit, offset)
            .await
            .unwrap();

        assert_eq!(res.offset, want_offset);
        assert_eq!(res.total, want_total);
        assert!(res.items == want_vec);
    }

    #[tokio::test]
    async fn get_items_should_filter_by_price() {
        let mut mock_repository = MockRepository::new();
        let mock_items = get_test_data();

        let query = Some(GetItemsQuery {
            ids: None,
            price: Some(PriceGetItemsQuery {
                from: Some(0.0),
                to: Some(100.0),
            }),
            names: None,
        });
        let sort = None;
        let limit = 0;
        let offset = 0;

        let want_vec = vec![mock_items[1].clone().into()];
        let want_offset = 0;
        let want_total = 1;

        mock_repository.expect_get_items().return_once(
            move |q: Option<GetItemsQuery>, s: Option<GetItemsSortBy>, lim: i64, off: i64| -> _ {
                if let Some(query) = Some(q.clone()) {
                    assert_eq!(q, query);
                }

                if let Some(sort) = Some(s.clone()) {
                    assert_eq!(s, sort);
                }

                assert_eq!(lim, limit);
                assert_eq!(off, offset);

                Box::pin(async move { Ok(vec![mock_items[1].clone()]) })
            },
        );

        let core_service = CoreServiceImpl::new(Arc::new(mock_repository));

        let res = core_service
            .get_items(query, sort, limit, offset)
            .await
            .unwrap();

        assert_eq!(res.offset, want_offset);
        assert_eq!(res.total, want_total);
        assert_eq!(res.items, want_vec);
    }

    #[tokio::test]
    async fn create_item_should_create_item() {
        let mut mock_repository = MockRepository::new();

        let item = Item {
            id: 0,
            name: "Q".to_string(),
            description: "S".to_string(),
            price: 100.0,
            sizes: vec![],
        };

        let item_backup = item.clone();
        mock_repository
            .expect_create_item()
            .return_once(move |i| {
                assert_eq!(i, item_backup);

                Box::pin(async move { Ok(1) })
            });

        let core_service = CoreServiceImpl::new(Arc::new(mock_repository));

        let res = core_service.create_item(item).await.unwrap();

        assert_eq!(res, 1);
    }

    #[tokio::test]
    async fn create_item_repo_error() {
        let mut mock_repository = MockRepository::new();

        let item = Item {
            id: 0,
            name: "Q".to_string(),
            description: "S".to_string(),
            price: 100.0,
            sizes: vec![],
        };

        let item_backup = item.clone();
        mock_repository
            .expect_create_item()
            .return_once(move |i| {
                assert_eq!(i, item_backup);

                Box::pin(async move { Err(RepositoryError{ message: "x".to_string(), code: RepositoryErrorKind::Unknown }) })
            });

        let core_service = CoreServiceImpl::new(Arc::new(mock_repository));

        let res = core_service.create_item(item).await.err().unwrap();

        assert_eq!(res, CommonError{ message: "x".to_string(), code: CommonErrorKind::Unknown });
    }
}
