// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use linera_sdk::{base::ChainId, util::BlockingWait, views::View, Contract, ContractRuntime};
use test_strategy::proptest;

use depin_demo::Operation;

use super::{DepinDemoContract, DepinDemoState};

/// Test initial state of the application.
#[test]
fn initial_state() {
    let app = create_and_instantiate_app();

    assert_eq!(*app.state.value.get(), 0);
    assert_eq!(*app.state.parent.get(), None);
}

/// Test if submitted new values accumulate in the state value.
#[proptest]
fn submit_operation(values_to_submit: Vec<u32>) {
    let mut app = create_and_instantiate_app();

    for &value in &values_to_submit {
        app.execute_operation(Operation::Submit {
            value: value.into(),
        })
        .blocking_wait();
    }

    assert_eq!(
        *app.state.value.get(),
        values_to_submit.into_iter().map(u64::from).sum::<u64>()
    );
}

/// Test that value overflows are rejected.
#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn submit_operation_overflow() {
    let mut app = create_and_instantiate_app();

    app.execute_operation(Operation::Submit { value: u64::MAX })
        .blocking_wait();

    app.execute_operation(Operation::Submit { value: 1 })
        .blocking_wait();
}

/// Test connecting the application to a parent chain.
#[proptest]
fn connect_to_parent(parent: ChainId) {
    let mut app = create_and_instantiate_app();

    app.execute_operation(Operation::ConnectToParent { parent })
        .blocking_wait();

    assert_eq!(*app.state.parent.get(), Some(parent));
}

/// Test if flushing without a configured parent causes the block to be rejected.
#[test]
#[should_panic(expected = "Can't flush if the chain is not connected to a parent chain")]
fn flush_without_parent() {
    let mut app = create_and_instantiate_app();

    app.execute_operation(Operation::Flush).blocking_wait();
}

/// Creates a [`DepinDemoContract`] instance ready to be tested.
fn create_and_instantiate_app() -> DepinDemoContract {
    let runtime = ContractRuntime::new().with_application_parameters(());
    let mut contract = DepinDemoContract {
        state: DepinDemoState::load(runtime.root_view_storage_context())
            .blocking_wait()
            .expect("Failed to read from mock key value store"),
        runtime,
    };

    contract.instantiate(()).blocking_wait();

    contract
}
