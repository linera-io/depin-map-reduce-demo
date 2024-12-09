// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use async_graphql::{Request, Response, Value};
use linera_sdk::{util::BlockingWait, views::View, Service, ServiceRuntime};
use serde_json::json;

use super::{DepinDemoService, DepinDemoState};

/// Test reading the value in the state.
#[test]
fn value_query() {
    let value = 60u64;
    let mut service = create_service();

    service.state.edit().value.set(value);

    let request = Request::new("{ value }");
    let response = service.handle_query(request).blocking_wait();

    let expected = Response::new(Value::from_json(json!({"value": 60})).unwrap());

    assert_eq!(response, expected)
}

/// Creates a [`DepinDemoService`] instance ready to be tested.
fn create_service() -> DepinDemoService {
    let runtime = ServiceRuntime::new();
    let state = DepinDemoState::load(runtime.root_view_storage_context())
        .blocking_wait()
        .expect("Failed to read from mock key value store");

    DepinDemoService {
        state: Arc::new(state),
        runtime,
    }
}
