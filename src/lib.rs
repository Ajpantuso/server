// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

mod server;

use clap::{Parser, Subcommand};
use std::{net::SocketAddr, path::PathBuf};

pub struct Command<'a> {
    options: &'a Options,
}

impl<'a> Command<'a> {
    pub fn from_options(options: &'a Options) -> Self {
        Self { options }
    }
    pub async fn run(&self) -> anyhow::Result<()> {
        match &self.options.sub_options {
            SubOptions::Serve {
                listen_addr,
                cert_path,
                key_path,
            } => {
                let mut builder = server::build(*listen_addr);

                if let Some(path) = cert_path {
                    builder = builder.cert_path(path)
                }
                if let Some(path) = key_path {
                    builder = builder.key_path(path)
                }

                let srv = builder.build();

                srv.serve().await
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
        #[arg(long = "key-path")]
        #[arg(requires = "cert_path")]
        key_path: Option<PathBuf>,
        #[arg(long = "cert-path")]
        #[arg(requires = "key_path")]
        cert_path: Option<PathBuf>,
    },
}
