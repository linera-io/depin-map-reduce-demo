// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

#[cfg(test)]
#[path = "unit_tests/contract.rs"]
mod tests;

use linera_sdk::{
    base::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime,
};

use depin_demo::Operation;

use self::state::DepinDemoState;

pub struct DepinDemoContract {
    state: DepinDemoState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(DepinDemoContract);

impl WithContractAbi for DepinDemoContract {
    type Abi = depin_demo::DepinDemoAbi;
}

impl Contract for DepinDemoContract {
    type Message = ();
    type Parameters = ();
    type InstantiationArgument = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = DepinDemoState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        DepinDemoContract { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        // validate that the application parameters were configured correctly.
        self.runtime.application_parameters();
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            Operation::Increment { value } => {
                self.state.value.set(self.state.value.get() + value);
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {}

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}
