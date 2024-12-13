// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Integration testing for the depin_demo application.

#![cfg(not(target_arch = "wasm32"))]

use depin_demo::Operation;
use futures::{stream, FutureExt, StreamExt, TryStreamExt};
use linera_sdk::test::TestValidator;

/// Tests producing values from multiple chains and propagating to the root.
#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn propagation_test() -> anyhow::Result<()> {
    const BRANCH_CHAINS: u64 = 5;
    const EDGE_CHAINS_PER_BRANCH: u64 = 10;

    let (validator, application_id, root_chain) =
        TestValidator::with_current_application::<depin_demo::DepinDemoAbi, _, _>((), ()).await;
    let root_chain_id = root_chain.id();

    stream::iter(0..BRANCH_CHAINS)
        .then(|branch_index| {
            let validator = validator.clone();
            tokio::spawn(async move {
                let branch_chain = validator.new_chain().await;
                let branch_chain_id = branch_chain.id();

                branch_chain.register_application(application_id).await;
                branch_chain
                    .add_block(|block| {
                        block.with_operation(
                            application_id,
                            Operation::ConnectToParent {
                                parent: root_chain_id,
                            },
                        );
                    })
                    .await;

                stream::iter(0..EDGE_CHAINS_PER_BRANCH)
                    .then(|edge_index| {
                        let validator = validator.clone();
                        tokio::spawn(async move {
                            let edge_chain = validator.new_chain().await;

                            edge_chain.register_application(application_id).await;
                            edge_chain
                                .add_block(|block| {
                                    block.with_operation(
                                        application_id,
                                        Operation::ConnectToParent {
                                            parent: branch_chain_id,
                                        },
                                    );
                                })
                                .await;
                            edge_chain
                                .add_block(|block| {
                                    block.with_operation(
                                        application_id,
                                        Operation::Submit {
                                            value: EDGE_CHAINS_PER_BRANCH * branch_index
                                                + edge_index,
                                        },
                                    );
                                })
                                .await;
                            edge_chain
                                .add_block(|block| {
                                    block.with_operation(application_id, Operation::Flush);
                                })
                                .await;
                        })
                    })
                    .try_collect::<()>()
                    .await?;

                branch_chain.handle_received_messages().await;
                branch_chain
                    .add_block(|block| {
                        block.with_operation(application_id, Operation::Flush);
                    })
                    .await;

                Ok::<_, anyhow::Error>(())
            })
            .map(|result| result.map_err(anyhow::Error::from).unwrap_or_else(Err))
        })
        .try_collect::<()>()
        .await?;

    root_chain.handle_received_messages().await;

    let response = root_chain
        .graphql_query(application_id, "query { value }")
        .await;
    let final_value = response["value"]
        .as_u64()
        .expect("Failed to get the value as `u64`");

    assert_eq!(
        final_value,
        (0..(EDGE_CHAINS_PER_BRANCH * BRANCH_CHAINS)).sum::<u64>()
    );

    Ok(())
}
