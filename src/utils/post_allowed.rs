use async_trait::async_trait;
use lazy_static::lazy_static;
use reqwest::redirect::Policy;
use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};
use trust_dns_resolver::{lookup_ip::LookupIp, TokioAsyncResolver};
use url::{Host, Url};

use crate::{error::Error, CONFIG};

lazy_static! {
    static ref RESOLVER: TokioAsyncResolver = TokioAsyncResolver::tokio_from_system_conf().unwrap();
}

pub async fn post_allowed<'a>(
    url: Url,
    body: &[(&'a str, &'a str)],
) -> Result<reqwest::Response, Error> {
    let client = if CONFIG.is_endpoint_allowed_by_user(&url) {
        reqwest::ClientBuilder::new().redirect(Policy::none())
    } else {
        let port = match url.port() {
            Some(p) => p,
            None if url.scheme() == "http" => 80,
            None if url.scheme() == "https" => 443,
            _ => return Err(Error::SchemeNotAllowed),
        };
        let resolved_socket_addrs = url
            .resolve_allowed()
            .await?
            .into_iter()
            .map(|ip| SocketAddr::new(ip, port))
            .collect::<Vec<SocketAddr>>();

        if resolved_socket_addrs.len().eq(&0) {
            log::info!(
                "Ignoring request to {}: no allowed ip",
                url.host_str().unwrap_or(&"No host")
            );
            return Err(Error::HostNotAllowed);
        }

        reqwest::ClientBuilder::new()
            .redirect(Policy::none())
            .no_trust_dns()
            .resolve_to_addrs(url.host_str().unwrap(), dbg!(&resolved_socket_addrs))
    }
    .build()
    .unwrap();

    Ok(client.post(url).json(&body).send().await?)
}

#[async_trait(?Send)]
pub trait ResolveAllowed {
    async fn resolve_allowed(&self) -> Result<Vec<IpAddr>, Error>;
}

#[async_trait(?Send)]
impl ResolveAllowed for Url {
    async fn resolve_allowed(&self) -> Result<Vec<IpAddr>, Error> {
        dbg!(&self);
        if ["http", "https"].contains(&self.scheme()) {
            self.host().unwrap().resolve_allowed().await
        } else {
            Err(Error::SchemeNotAllowed)
        }
    }
}

#[async_trait(?Send)]
impl ResolveAllowed for Host<&str> {
    async fn resolve_allowed(&self) -> Result<Vec<IpAddr>, Error> {
        match self {
            Host::Domain(d) => {
                RESOLVER
                    .lookup_ip(*d)
                    .await
                    .unwrap()
                    .resolve_allowed()
                    .await
            }
            Host::Ipv4(ip) if ip.is_global() => Ok(vec![IpAddr::V4(ip.clone())]),
            Host::Ipv6(ip) if ip.is_global() => Ok(vec![IpAddr::V6(ip.clone())]),
            _ => Err(Error::HostNotAllowed),
        }
    }
}

#[async_trait(?Send)]
impl ResolveAllowed for LookupIp {
    async fn resolve_allowed(&self) -> Result<Vec<IpAddr>, Error> {
        Ok(self.iter().filter(|ip| ip.is_global()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn len_from_str(url: &str) -> usize {
        Url::from_str(url)
            .unwrap()
            .resolve_allowed()
            .await
            .unwrap()
            .len()
    }

    #[tokio::test]
    async fn test_post() {
        post_allowed(
            Url::from_str("https://httpbin.org/post").unwrap(),
            &[("foo", "blah")],
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
