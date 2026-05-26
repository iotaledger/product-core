// Copyright 2020-2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use futures::stream::Stream;
use iota_sdk::{
    graphql_client::{
        Client as IotaClient, Page, error::Error as IotaClientError, query_types::ObjectFilter,
        streams::stream_paginated_query,
    },
    types::ObjectId,
};
use serde::de::DeserializeOwned;

use crate::{move_type::MoveType, network::Network};

pub trait ProductClient: Deref<Target = IotaClient> + Sized {
    fn network(&self) -> Network;
    fn package_id(&self) -> ObjectId;

    fn objects_content_stream<'a, T>(
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
    let pagination = client.pagination_filter(pagination_filter).await;
    let operation = ObjectsQuery::build(ObjectsQueryArgs {
        after: pagination.after,
        before: pagination.before,
        filter: Some(filter),
        first: pagination.first,
        last: pagination.last,
    });

    let response = client.run_query(&operation).await?;

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
