// Copyright 2020-2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::pin::pin;

use futures::{StreamExt, TryStreamExt, stream::Stream};
use iota_sdk::{
    graphql_client::{
        Client as IotaClient, Page, error::Error as IotaClientError, query_types::ObjectFilter,
        streams::stream_paginated_query,
    },
    types::{Address, ObjectId},
};
use serde::de::DeserializeOwned;

use crate::{move_type::MoveType, network::Network, type_origin_table::TypeOriginTable};

pub trait ProductClient: AsRef<IotaClient> + Sized + Send + Sync {
    fn network(&self) -> Network;
    fn type_origin_table(&self) -> &TypeOriginTable;
    fn package_id(&self) -> ObjectId;

    fn objects_content_stream<T>(
        &self,
        filter: impl Into<Option<ObjectFilter>>,
    ) -> impl Stream<Item = Result<T, IotaClientError>>
    where
        T: DeserializeOwned + MoveType + Clone + Unpin,
    {
        use iota_sdk::graphql_client::Direction;

        let filter = filter.into().unwrap_or_default();
        stream_paginated_query(
            move |page_info| objects_content_paginatated(self, filter.clone(), page_info.cursor),
            Direction::Forward,
        )
    }

    fn find_object_for_address<'a, T, F>(
        &'a self,
        address: Address,
        pred: F,
    ) -> impl Future<Output = Result<Option<T>, IotaClientError>>
    where
        T: DeserializeOwned + MoveType + Clone + Unpin,
        F: Fn(&T) -> bool + 'a,
    {
        async move {
            let object_stream = self
                .objects_content_stream::<T>(
                    ObjectFilter::default()
                        .with_owner(address)
                        .with_type(T::move_type(self).to_string()),
                )
                .try_filter(|obj| std::future::ready(pred(obj)));
            pin!(object_stream).next().await.transpose()
        }
    }
}

async fn objects_content_paginatated<T>(
    client: &impl ProductClient,
    filter: ObjectFilter,
    cursor: Option<String>,
) -> Result<Page<T>, IotaClientError>
where
    T: DeserializeOwned,
{
    use cynic::QueryBuilder;
    use iota_sdk::graphql_client::{
        Direction, PaginationFilter,
        query_types::{ObjectsQuery, ObjectsQueryArgs},
    };

    let pagination_filter = PaginationFilter {
        direction: Direction::Forward,
        cursor,
        limit: None,
    };
    let pagination = client.as_ref().pagination_filter(pagination_filter).await;
    let operation = ObjectsQuery::build(ObjectsQueryArgs {
        after: pagination.after,
        before: pagination.before,
        filter: Some(filter),
        first: pagination.first,
        last: pagination.last,
    });

    let response = client.as_ref().run_query(&operation).await?;

    let oc = response.objects;
    let page_info = oc.page_info;
    let objects = oc
        .nodes
        .into_iter()
        .filter_map(|object| {
            object
                .as_move_object
                .and_then(|move_object| move_object.contents)
                .and_then(|move_value| move_value.json)
        })
        .flat_map(serde_json::from_value)
        .collect();

    Ok(Page::new(page_info, objects))
}
