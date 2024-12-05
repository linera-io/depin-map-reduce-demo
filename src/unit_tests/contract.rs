// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use linera_sdk::{util::BlockingWait, views::View, Contract, ContractRuntime};

use depin_demo::Operation;

use super::{DepinDemoContract, DepinDemoState};

/// Test initial state of the application.
#[test]
fn initial_state() {
    let app = create_and_instantiate_app();

    assert_eq!(*app.state.value.get(), 0);
}

#[test]
fn operation() {
    let mut app = create_and_instantiate_app();

    let increment = 10u64;

    let _response = app
        .execute_operation(Operation::Increment { value: increment })
        .blocking_wait();

    assert_eq!(*app.state.value.get(), increment);
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
