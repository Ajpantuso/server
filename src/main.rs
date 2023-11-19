// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use server::{Command, Options};

#[tokio::main]
async fn main() {
    let opts = Options::parse();

    Command::from_options(&opts).run().await;
}
