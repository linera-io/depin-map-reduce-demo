#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use async_graphql::{EmptySubscription, Object, Schema};
use linera_sdk::{
    base::WithServiceAbi, graphql::GraphQLMutationRoot, views::View, Service, ServiceRuntime,
};

use depin_demo::Operation;

use self::state::DepinDemoState;

pub struct DepinDemoService {
    state: DepinDemoState,
    runtime: ServiceRuntime<Self>,
}

linera_sdk::service!(DepinDemoService);

impl WithServiceAbi for DepinDemoService {
    type Abi = depin_demo::DepinDemoAbi;
}

impl Service for DepinDemoService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = DepinDemoState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        DepinDemoService { state, runtime }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        Schema::build(
            QueryRoot {
                value: *self.state.value.get(),
            },
            Operation::mutation_root(),
            EmptySubscription,
        )
        .finish()
        .execute(query)
        .await
    }
}

struct QueryRoot {
    value: u64,
}

#[Object]
impl QueryRoot {
    async fn value(&self) -> &u64 {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use async_graphql::{Request, Response, Value};
    use futures::FutureExt as _;
    use linera_sdk::{util::BlockingWait, views::View, Service, ServiceRuntime};
    use serde_json::json;

    use super::{DepinDemoService, DepinDemoState};

    #[test]
    fn query() {
        let value = 60u64;
        let runtime = ServiceRuntime::<DepinDemoService>::new();
        let mut state = DepinDemoState::load(runtime.root_view_storage_context())
            .blocking_wait()
            .expect("Failed to read from mock key value store");
        state.value.set(value);

        let service = DepinDemoService { state, runtime };
        let request = Request::new("{ value }");

        let response = service
            .handle_query(request)
            .now_or_never()
            .expect("Query should not await anything");

        let expected = Response::new(Value::from_json(json!({"value": 60})).unwrap());

        assert_eq!(response, expected)
    }
}
