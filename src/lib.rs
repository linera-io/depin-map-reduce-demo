// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::{Request, Response};
use linera_sdk::{
    linera_base_types::ChainId,
    abi::{ContractAbi, ServiceAbi},
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};

pub struct DepinDemoAbi;

impl ContractAbi for DepinDemoAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for DepinDemoAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    ConnectToParent { parent: ChainId },
    Submit { value: u64 },
    Flush,
}
