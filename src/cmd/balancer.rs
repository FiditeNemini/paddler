use pingora::{
    proxy::http_proxy_service,
    server::{configuration::Opt, Server},
};
use std::{net::SocketAddr, sync::Arc, time::Duration};

use crate::balancer::management_service::ManagementService;
use crate::balancer::proxy_service::ProxyService;
use crate::balancer::statsd_service::StatsdService;
use crate::balancer::upstream_peer_pool::UpstreamPeerPool;
use crate::errors::result::Result;

pub fn handle(
    management_addr: &SocketAddr,
    management_dashboard_enable: bool,
    reverseproxy_addr: &SocketAddr,
    rewrite_host_header: bool,
    slots_endpoint_enable: bool,
    statsd_addr: Option<SocketAddr>,
    statsd_prefix: String,
    statsd_reporting_interval: Duration,
) -> Result<()> {
    let mut pingora_server = Server::new(Opt {
        upgrade: false,
        daemon: false,
        nocapture: false,
        test: false,
        conf: None,
    })?;

    pingora_server.bootstrap();

    let upstream_peer_pool = Arc::new(UpstreamPeerPool::new());

    let mut proxy_service = http_proxy_service(
        &pingora_server.configuration,
        ProxyService::new(
            rewrite_host_header,
            slots_endpoint_enable,
            upstream_peer_pool.clone(),
        ),
    );

    proxy_service.add_tcp(&reverseproxy_addr.clone().to_string());

    pingora_server.add_service(proxy_service);
    pingora_server.add_service(ManagementService::new(
        *management_addr,
        management_dashboard_enable,
        upstream_peer_pool.clone(),
    ));

    if let Some(statsd_addr) = statsd_addr {
        let statsd_service = StatsdService::new(
            statsd_addr,
            statsd_prefix,
            statsd_reporting_interval,
            upstream_peer_pool.clone(),
        )?;

        pingora_server.add_service(statsd_service);
    }

    pingora_server.run_forever();
}
