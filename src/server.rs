// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use log::{debug, info};
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
    pub fn cert_path(self, cert_path: &'a impl AsRef<Path>) -> Self {
        Self {
            cert_path: Some(cert_path.as_ref()),
            ..self
        }
    }
    pub fn key_path(self, key_path: &'a impl AsRef<Path>) -> Self {
        Self {
            key_path: Some(key_path.as_ref()),
            ..self
        }
    }
    pub fn build(self) -> Server<'a> {
        match (self.cert_path, self.key_path) {
            (Some(cert_path), Some(key_path)) => Server {
                address: self.listen_addr,
                tls_options: Some(TLSOptions {
                    key_path,
                    cert_path,
                }),
            },
            _ => Server {
                address: self.listen_addr,
                tls_options: None,
            },
        }
    }
}

pub struct Server<'a> {
    address: SocketAddr,
    tls_options: Option<TLSOptions<'a>>,
}

impl<'a> Server<'a> {
    pub async fn serve(&self) -> anyhow::Result<()> {
        let log = warp::log("server::site");

        debug!("loading index.html");

        let raw_index = Assets::get("html/index.html").unwrap();
        let index = std::str::from_utf8(raw_index.data.as_ref())?.to_string();

        debug!("registering routes");

        let root = warp::path::end()
            .map(move || warp::reply::html(index.clone()))
            .or(warp::path!("healthz").and(healthz()))
            .or(warp::path!("readyz").and(readyz()))
            .with(log);

        info!("starting server on address {}", self.address);

        match &self.tls_options {
            Some(opts) => {
                debug!(
                    "using TLS with key {} and cert {}",
                    opts.key_path.display(),
                    opts.cert_path.display()
                );

                let srv = warp::serve(root)
                    .tls()
                    .key_path(opts.key_path)
                    .cert_path(opts.cert_path);
                let (_addr, srv) = srv.bind_with_graceful_shutdown(self.address, async move {
                    signal::ctrl_c()
                        .await
                        .expect("failed to shutdown gracefully")
                });

                srv.await;
            }
            None => {
                let srv = warp::serve(root);
                let (_addr, srv) = srv.bind_with_graceful_shutdown(self.address, async move {
                    signal::ctrl_c()
                        .await
                        .expect("failed to shutdown gracefully")
                });

                srv.await;
            }
        }

        Ok(())
    }
}

fn healthz() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
    warp::get().map(|| "ok".into())
}

fn readyz() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
    warp::get().map(|| "ok".into())
}

#[derive(RustEmbed)]
#[folder = "assets"]
struct Assets;

struct TLSOptions<'a> {
    key_path: &'a Path,
    cert_path: &'a Path,
}

#[cfg(test)]
mod tests {
    use super::{healthz, readyz};
    use warp::test::request;

    #[tokio::test]
    async fn healthz_reply() {
        let filter = healthz();
        let res = request().method("GET").reply(&filter).await;

        assert_eq!(200, res.status());
        assert_eq!("ok", res.body());
    }
    #[tokio::test]
    async fn readyz_reply() {
        let filter = readyz();
        let res = request().method("GET").reply(&filter).await;

        assert_eq!(200, res.status());
        assert_eq!("ok", res.body());
    }
}
