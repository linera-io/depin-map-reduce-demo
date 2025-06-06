// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use async_graphql::{Request, Response, Value};
use linera_sdk::{linera_base_types::ChainId, util::BlockingWait, views::View, Service, ServiceRuntime};
use serde_json::json;
use test_strategy::proptest;

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

/// Test if it's possible to read the value in the state.
#[test]
fn empty_parent_query() {
    let service = create_service();

    let request = Request::new("{ parent }");
    let response = service.handle_query(request).blocking_wait();

    let expected = Response::new(Value::from_json(json!({"parent": null})).unwrap());

    assert_eq!(response, expected)
}

/// Test if it's possible to read the value in the state.
#[proptest]
fn parent_query(parent: ChainId) {
    let mut service = create_service();

    service.state.edit().parent.set(Some(parent));

    let request = Request::new("{ parent }");
    let response = service.handle_query(request).blocking_wait();

    let expected =
        Response::new(Value::from_json(json!({ "parent": parent.to_string() })).unwrap());

    assert_eq!(response, expected)
}

/// Test creating a connect to parent operation.
#[proptest]
fn connect_to_parent_mutation(parent: ChainId) {
    let service = create_service();
    let request = Request::new(format!(
        "mutation {{ connectToParent(parent: \"{parent}\") }}"
    ));
    let response = service.handle_query(request).blocking_wait();
    let expected = Response::new(Value::from_json(json!({"connectToParent": true})).unwrap());
    assert_eq!(response, expected);
}

/// Test creating a submit operation.
#[proptest]
fn submit_mutation(value: u64) {
    let service = create_service();
    let request = Request::new(format!("mutation {{ submit(value: \"{value}\") }}"));
    let response = service.handle_query(request).blocking_wait();
    let expected = Response::new(Value::from_json(json!({"submit": true})).unwrap());
    assert_eq!(response, expected);
}

/// Test creating a flush operation.
#[test]
fn flush_mutation() {
    let service = create_service();
    let request = Request::new("mutation { flush }");
    let response = service.handle_query(request).blocking_wait();
    let expected = Response::new(Value::from_json(json!({"flush": true})).unwrap());
    assert_eq!(response, expected);
}

/// Creates a [`DepinDemoService`] instance ready to be tested.
fn create_service() -> DepinDemoService {
    let runtime = ServiceRuntime::new();
    let state = DepinDemoState::load(runtime.root_view_storage_context())
        .blocking_wait()
        .expect("Failed to read from mock key value store");

    DepinDemoService {
        state: Arc::new(state),
        runtime: Arc::new(runtime),
    }
}
