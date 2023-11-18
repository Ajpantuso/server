// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use crate::server::Server;
use clap::{Parser, Subcommand};
use std::net::SocketAddr;

pub struct Command<'a> {
    options: &'a Options,
}

impl<'a> Command<'a> {
    pub fn from_options(opts: &'a Options) -> Self {
        Self { options: opts }
    }
    pub async fn run(&self) {
        match self.options.sub_options {
            SubOptions::Serve { listen_addr } => {
                let srv = Server::new(listen_addr);

                srv.serve().await;
            }
        }
    }
}

#[derive(Parser)]
pub struct Options {
    #[command(subcommand)]
    pub sub_options: SubOptions,
}

#[derive(Subcommand)]
pub enum SubOptions {
    Serve {
        #[arg(long = "listen-addr")]
        #[arg(default_value = "127.0.0.1:8080")]
        listen_addr: SocketAddr,
    },
}
