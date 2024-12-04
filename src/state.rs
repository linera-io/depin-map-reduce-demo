// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

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
