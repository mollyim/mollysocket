use async_trait::async_trait;
use eyre::{eyre, Result};
use lazy_static::lazy_static;
use reqwest::redirect::Policy;
use serde::Serialize;
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter},
    net::{IpAddr, SocketAddr},
};
use trust_dns_resolver::{lookup_ip::LookupIp, TokioAsyncResolver};
use url::{Host, Url};

use crate::config;

lazy_static! {
    static ref RESOLVER: TokioAsyncResolver = TokioAsyncResolver::tokio_from_system_conf().unwrap();
}

#[derive(Debug)]
enum Error {
    SchemeNotAllowed,
    HostNotAllowed,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl StdError for Error {}

pub async fn post_allowed<T: Serialize + ?Sized>(url: Url, body: &T) -> Result<reqwest::Response> {
    let port = match url.port() {
        Some(p) => p,
        None if url.scheme() == "http" => 80,
        None if url.scheme() == "https" => 443,
        _ => return Err(eyre!(Error::SchemeNotAllowed)),
    };

    let client = if config::is_endpoint_allowed_by_user(&url) {
        reqwest::ClientBuilder::new().redirect(Policy::none())
    } else {
        let resolved_socket_addrs = url
            .resolve_allowed()
            .await?
            .into_iter()
            .map(|ip| SocketAddr::new(ip, port))
            .collect::<Vec<SocketAddr>>();

        if resolved_socket_addrs.is_empty() {
            log::info!(
                "Ignoring request to {}: no allowed ip",
                url.host_str().unwrap_or("No host")
            );
            return Err(eyre!(Error::HostNotAllowed));
        }

        reqwest::ClientBuilder::new()
            .redirect(Policy::none())
            .no_trust_dns()
            .resolve_to_addrs(url.host_str().unwrap(), &resolved_socket_addrs)
    }
    .build()
    .unwrap();

    Ok(client.post(url).json(&body).send().await?)
}

#[async_trait]
pub trait ResolveAllowed {
    async fn resolve_allowed(&self) -> Result<Vec<IpAddr>>;
}

#[async_trait]
impl ResolveAllowed for Url {
    async fn resolve_allowed(&self) -> Result<Vec<IpAddr>> {
        if ["http", "https"].contains(&self.scheme()) {
            self.host()
                .ok_or(Error::HostNotAllowed)?
                .resolve_allowed()
                .await
        } else {
            Err(eyre!(Error::SchemeNotAllowed))
        }
    }
}

#[async_trait]
impl ResolveAllowed for Host<&str> {
    async fn resolve_allowed(&self) -> Result<Vec<IpAddr>> {
        match self {
            Host::Domain(d) => {
                RESOLVER
                    .lookup_ip(*d)
                    .await
                    .map_err(|_| Error::HostNotAllowed)?
                    .resolve_allowed()
                    .await
            }
            Host::Ipv4(ip) if ip_rfc::global_v4(ip) => Ok(vec![IpAddr::V4(*ip)]),
            Host::Ipv6(ip) if ip_rfc::global_v6(ip) => Ok(vec![IpAddr::V6(*ip)]),
            _ => Err(eyre!(Error::HostNotAllowed)),
        }
    }
}

#[async_trait]
impl ResolveAllowed for LookupIp {
    async fn resolve_allowed(&self) -> Result<Vec<IpAddr>> {
        Ok(self.iter().filter(ip_rfc::global).collect())
    }
}

#[cfg(test)]
mod tests {
    use rocket::serde::json::serde_json::json;

    use super::*;
    use std::str::FromStr;

    async fn len_from_str(url: &str) -> usize {
        Url::from_str(url)
            .unwrap()
            .resolve_allowed()
            .await
            .unwrap_or(vec![])
            .len()
    }

    #[tokio::test]
    async fn test_post() {
        post_allowed(
            Url::from_str("https://httpbin.org/post").unwrap(),
            &json!({"urgent": true}),
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_not_allowed() {
        assert_eq!(len_from_str("unix://signal.org").await, 0);
        assert_eq!(len_from_str("http://127.1").await, 0);
        assert_eq!(len_from_str("http://localhost").await, 0);
        assert_eq!(len_from_str("http://[::1]").await, 0);
        assert_eq!(len_from_str("http://10.10.1.1").await, 0);
        assert_eq!(len_from_str("http://[fc01::2]").await, 0);
    }

    #[tokio::test]
    async fn test_allowed() {
        assert!(len_from_str("http://signal.org").await.gt(&0));
        assert!(len_from_str("http://signal.org:8080").await.gt(&0));
        assert!(len_from_str("https://signal.org").await.gt(&0));
        assert!(len_from_str("http://18.244.114.115").await.gt(&0));
        assert!(
            len_from_str("http://[2600:9000:2550:ae00:13:5d53:5740:93a1]")
                .await
                .gt(&0)
        );
    }
}
