// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::mem;

use linera_sdk::{
    base::{ChainId, Destination, Resources, SendMessageRequest},
    util::BlockingWait,
    views::View,
    Contract, ContractRuntime,
};
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

/// Test if flushing values sends messages to the parent chain.
#[proptest]
fn flush_sends_messages(parent: ChainId, values_to_submit: Vec<Option<u32>>) {
    let mut app = create_and_instantiate_app();
    let mut accumulated = 0_u64;

    app.execute_operation(Operation::ConnectToParent { parent })
        .blocking_wait();

    for maybe_value in values_to_submit.into_iter().chain(None) {
        match maybe_value {
            Some(value) => {
                app.execute_operation(Operation::Submit {
                    value: value.into(),
                })
                .blocking_wait();

                accumulated += u64::from(value);
            }
            None => {
                app.execute_operation(Operation::Flush).blocking_wait();

                assert_eq!(
                    mem::take(&mut *app.runtime.created_send_message_requests()),
                    vec![SendMessageRequest {
                        destination: Destination::Recipient(parent),
                        authenticated: false,
                        is_tracked: false,
                        grant: Resources::default(),
                        message: mem::take(&mut accumulated),
                    }]
                );
            }
        }
    }
}

/// Test that value overflows are avoided by flushing.
#[proptest]
fn submit_operation_overflow_is_avoided_by_flushing(parent: ChainId) {
    let mut app = create_and_instantiate_app();

    app.execute_operation(Operation::Submit { value: u64::MAX })
        .blocking_wait();

    app.execute_operation(Operation::ConnectToParent { parent })
        .blocking_wait();
    app.execute_operation(Operation::Flush).blocking_wait();

    app.execute_operation(Operation::Submit { value: 1 })
        .blocking_wait();

    assert_eq!(*app.state.value.get(), 1);
}

/// Test if flushed values are accumulated.
#[proptest]
fn incoming_messages_are_accumulated(incoming_messages: Vec<u32>) {
    let mut app = create_and_instantiate_app();

    for &message in &incoming_messages {
        app.execute_message(message.into()).blocking_wait();
    }

    assert_eq!(
        *app.state.value.get(),
        incoming_messages.into_iter().map(u64::from).sum::<u64>()
    );
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
