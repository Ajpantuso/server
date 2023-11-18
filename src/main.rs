// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

mod command;
mod server;

use clap::Parser;

#[tokio::main]
async fn main() {
    let opts = command::Options::parse();

    command::Command::from_options(&opts).run().await;
}
