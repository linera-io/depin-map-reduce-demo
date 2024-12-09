// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
use std::sync::Arc;

use linera_sdk::{
    base::ChainId,
    views::{linera_views, RegisterView, RootView, ViewStorageContext},
};

#[derive(RootView, async_graphql::SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct DepinDemoState {
    pub parent: RegisterView<Option<ChainId>>,
    pub value: RegisterView<u64>,
}

#[cfg(test)]
#[allow(dead_code)]
impl DepinDemoState {
    /// Allows editing the state in a service implementation in tests.
    pub fn edit(self: &mut Arc<Self>) -> &mut Self {
        Arc::get_mut(self).expect("State can not be edited when it is being shared")
    }
}
