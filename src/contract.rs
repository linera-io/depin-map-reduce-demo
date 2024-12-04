// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

#[cfg(test)]
#[path = "unit_tests/contract.rs"]
mod tests;

use std::mem;

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
    type Message = u64;
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
            Operation::ConnectToParent { parent } => {
                self.state.parent.set(Some(parent));
            }
            Operation::Submit { value } => {
                self.state.value.set(self.state.value.get() + value);
            }
            Operation::Flush => {
                let parent = self
                    .state
                    .parent
                    .get()
                    .expect("Can't flush if the chain is not connected to a parent chain");
                let value = mem::take(self.state.value.get_mut());

                self.runtime.send_message(parent, value);
            }
        }
    }

    async fn execute_message(&mut self, child_value: Self::Message) {
        self.state.value.set(self.state.value.get() + child_value);
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}
