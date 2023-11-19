// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use rust_embed::RustEmbed;
use std::{net::SocketAddr, path::Path};
use tokio::signal;
use warp::Filter;

pub fn build<'a>(listen_addr: impl Into<SocketAddr>) -> ServerBuilder<'a> {
    ServerBuilder {
        listen_addr: listen_addr.into(),
        cert_path: None,
        key_path: None,
    }
}

pub struct ServerBuilder<'a> {
    listen_addr: SocketAddr,
    cert_path: Option<&'a Path>,
    key_path: Option<&'a Path>,
}

impl<'a> ServerBuilder<'a> {
    pub fn cert_path(mut self, cert_path: &'a impl AsRef<Path>) -> Self {
        self.cert_path = Some(cert_path.as_ref());

        self
    }
    pub fn key_path(mut self, key_path: &'a impl AsRef<Path>) -> Self {
        self.key_path = Some(key_path.as_ref());

        self
    }
    pub fn build(self) -> Server<'a> {
        if self.cert_path.is_some() && self.key_path.is_some() {
            Server {
                address: self.listen_addr,
                tls_options: Some(TLSOptions {
                    key_path: self.key_path.unwrap(),
                    cert_path: self.cert_path.unwrap(),
                }),
            }
        } else {
            Server {
                address: self.listen_addr,
                tls_options: None,
            }
        }
    }
}

pub struct Server<'a> {
    address: SocketAddr,
    tls_options: Option<TLSOptions<'a>>,
}

struct TLSOptions<'a> {
    key_path: &'a Path,
    cert_path: &'a Path,
}

impl<'a> Server<'a> {
    pub async fn serve(&self) {
        let raw_index = Assets::get("html/index.html").unwrap();
        let index = std::str::from_utf8(raw_index.data.as_ref())
            .unwrap()
            .to_string();

        let root = warp::path::end()
            .map(move || warp::reply::html(index.clone()))
            .or(warp::path!("readyz").map(readyz));

        match &self.tls_options {
            Some(opts) => {
                let srv = warp::serve(root)
                    .tls()
                    .key_path(opts.key_path)
                    .cert_path(opts.cert_path);
                let (_addr, srv) = srv.bind_with_graceful_shutdown(self.address, async move {
                    signal::ctrl_c()
                        .await
                        .expect("attempting graceful shutdown")
                });

                srv.await;
            }
            None => {
                let srv = warp::serve(root);
                let (_addr, srv) = srv.bind_with_graceful_shutdown(self.address, async move {
                    signal::ctrl_c()
                        .await
                        .expect("attempting graceful shutdown")
                });

                srv.await;
            }
        }
    }
}

fn readyz() -> String {
    "ok".into()
}

#[derive(RustEmbed)]
#[folder = "assets"]
struct Assets;
