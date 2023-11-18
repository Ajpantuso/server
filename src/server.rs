// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use rust_embed::RustEmbed;
use std::net::SocketAddr;
use tokio::signal;
use warp::Filter;

pub struct Server {
    address: SocketAddr,
}

impl Server {
    pub fn new(addr: impl Into<SocketAddr>) -> Self {
        Self {
            address: addr.into(),
        }
    }
    pub async fn serve(&self) {
        let raw_index = Assets::get("html/index.html").unwrap();
        let index = std::str::from_utf8(raw_index.data.as_ref())
            .unwrap()
            .to_string();

        let root = warp::path::end()
            .map(move || warp::reply::html(index.clone()))
            .or(warp::path!("readyz").map(readyz));

        let (_addr, srv) =
            warp::serve(root).bind_with_graceful_shutdown(self.address, async move {
                signal::ctrl_c()
                    .await
                    .expect("attempting graceful shutdown")
            });

        srv.await;
    }
}

fn readyz() -> String {
    "ok".into()
}

#[derive(RustEmbed)]
#[folder = "assets"]
struct Assets;
