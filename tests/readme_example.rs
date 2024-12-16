// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! An integration test to run the usage example in the README.

use std::{fs, iter, process::Command};

/// Run the example in the README file, checking that it completes successfully.
#[test]
fn readme_example_usage() -> anyhow::Result<()> {
    let readme_file = fs::read_to_string("README.md")?;

    let readme_commands = readme_file
        .lines()
        .skip_while(|line| *line != "## Example Usage")
        // Skip localnet setup
        .skip_while(|line| *line != "```ignore")
        .skip_while(|line| *line != "```")
        .skip(1)
        // Extract the script blocks
        .scan(false, |in_quote_block, line| {
            if line == "```" {
                *in_quote_block = !*in_quote_block;
                Some(None)
            } else if *in_quote_block {
                Some(Some(line))
            } else {
                Some(None)
            }
        })
        .flatten()
        // Add delays between commands
        .flat_map(|line| {
            if line.ends_with("&") {
                [line, "\nsleep 6\n"]
            } else if line.ends_with("\\") || line.starts_with('#') || line.trim().is_empty() {
                [line, "\n"]
            } else {
                [line, "\nsleep 1\n"]
            }
        });

    // Kill background processes when finished
    let exit_handler = "trap 'jobs -p | xargs -r kill' SIGINT SIGTERM EXIT\n";
    // Remove the temporary wallet directory if the commands succeed, assuming it's not empty
    let wallet_removal = "rm \"${LINERA_WALLET}\"\n\
         export LINERA_STORAGE_DIR=\"$(echo \"${LINERA_STORAGE}\" | cut -d: -f2)\"\n\
         rm -rf \"${LINERA_STORAGE_DIR}/table_linera\"\n\
         rmdir \"$LINERA_STORAGE_DIR\"\n\
         rmdir \"${WALLET_DIR}\"\n";

    let script = iter::once(exit_handler)
        .chain(readme_commands)
        .chain(iter::once(wallet_removal))
        .collect::<String>();

    assert!(Command::new("bash")
        .args(["-x", "-e", "-c", &script])
        .status()?
        .success());

    Ok(())
}
