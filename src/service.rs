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
    linera_base_types::ChainId,
    abi::WithServiceAbi,
    views::View,
    Service, ServiceRuntime,
};

use depin_demo::Operation;

use self::state::DepinDemoState;

pub struct DepinDemoService {
    state: Arc<DepinDemoState>,
    runtime: Arc<ServiceRuntime<Self>>,
}

linera_sdk::service!(DepinDemoService);

impl WithServiceAbi for DepinDemoService {
    type Abi = depin_demo::DepinDemoAbi;
}

impl Service for DepinDemoService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        DepinDemoService {
            state: Arc::new(
                DepinDemoState::load(runtime.root_view_storage_context())
                    .await
                    .expect("Failed to load state"),
            ),
            runtime: Arc::new(runtime),
        }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        Schema::build(
            self.state.clone(),
            MutationRoot {
                runtime: Arc::clone(&self.runtime),
            },
            EmptySubscription,
        )
            .finish()
            .execute(query)
            .await
    }
}

struct MutationRoot {
    runtime: Arc<ServiceRuntime<DepinDemoService>>,
}

#[async_graphql::Object]
impl MutationRoot {
    /// Creates an operation to connect this chain to a parent chain.
    async fn connect_to_parent(&self, parent: ChainId) -> bool {
        self.runtime.schedule_operation(&Operation::ConnectToParent { parent });
        true
    }

    /// Creates an operation to submit a value.
    async fn submit(&self, value: String) -> async_graphql::Result<bool> {
        self.runtime.schedule_operation(&Operation::Submit { value: value.parse()? });
        Ok(true)
   }

    /// Creates an operation to flush the accumulated values to the parent chain.
    async fn flush(&self) -> bool {
        self.runtime.schedule_operation(&Operation::Flush);
        true
    }
}
