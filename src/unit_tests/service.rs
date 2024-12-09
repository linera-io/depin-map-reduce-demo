// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use async_graphql::{Request, Response, Value};
use futures::FutureExt as _;
use linera_sdk::{util::BlockingWait, views::View, Service, ServiceRuntime};
use serde_json::json;

use super::{DepinDemoService, DepinDemoState};

/// Test reading the value in the state.
#[test]
fn value_query() {
    let value = 60u64;
    let runtime = ServiceRuntime::<DepinDemoService>::new();
    let mut state = DepinDemoState::load(runtime.root_view_storage_context())
        .blocking_wait()
        .expect("Failed to read from mock key value store");
    state.value.set(value);

    let service = DepinDemoService {
        state: Arc::new(state),
        runtime,
    };
    let request = Request::new("{ value }");

    let response = service
        .handle_query(request)
        .now_or_never()
        .expect("Query should not await anything");

    let expected = Response::new(Value::from_json(json!({"value": 60})).unwrap());

    assert_eq!(response, expected)
}
