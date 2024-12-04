// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

#[cfg(test)]
#[path = "unit_tests/service.rs"]
mod tests;

use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};
use linera_sdk::{
    base::WithServiceAbi, graphql::GraphQLMutationRoot, views::View, Service, ServiceRuntime,
};

use depin_demo::Operation;

use self::state::DepinDemoState;

pub struct DepinDemoService {
    state: Arc<DepinDemoState>,
    runtime: ServiceRuntime<Self>,
}

linera_sdk::service!(DepinDemoService);

impl WithServiceAbi for DepinDemoService {
    type Abi = depin_demo::DepinDemoAbi;
}

impl Service for DepinDemoService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = Arc::new(
            DepinDemoState::load(runtime.root_view_storage_context())
                .await
                .expect("Failed to load state"),
        );

        DepinDemoService { state, runtime }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        Schema::build(
            self.state.clone(),
            Operation::mutation_root(),
            EmptySubscription,
        )
        .finish()
        .execute(query)
        .await
    }
}
