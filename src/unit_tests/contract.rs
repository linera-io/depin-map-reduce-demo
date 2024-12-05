// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use linera_sdk::{util::BlockingWait, views::View, Contract, ContractRuntime};

use depin_demo::Operation;

use super::{DepinDemoContract, DepinDemoState};

#[test]
fn operation() {
    let initial_value = 10u64;
    let mut app = create_and_instantiate_app(initial_value);

    let increment = 10u64;

    let _response = app
        .execute_operation(Operation::Increment { value: increment })
        .blocking_wait();

    assert_eq!(*app.state.value.get(), initial_value + increment);
}

/// Creates a [`DepinDemoContract`] instance ready to be tested.
fn create_and_instantiate_app(initial_value: u64) -> DepinDemoContract {
    let runtime = ContractRuntime::new().with_application_parameters(());
    let mut contract = DepinDemoContract {
        state: DepinDemoState::load(runtime.root_view_storage_context())
            .blocking_wait()
            .expect("Failed to read from mock key value store"),
        runtime,
    };

    contract.instantiate(initial_value).blocking_wait();

    assert_eq!(*contract.state.value.get(), initial_value);

    contract
}
