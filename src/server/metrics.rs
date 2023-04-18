use std::fmt::Display;

use eyre::Result;
use rocket::{http::uri::Origin, Build, Rocket};
use rocket_prometheus::{
    prometheus::{register_int_counter, register_int_gauge, IntCounter, IntGauge},
    PrometheusMetrics,
};

pub struct Metrics {
    pub connections: IntGauge,
    pub reconnections: IntCounter,
    pub messages: IntCounter,
    pub pushs: IntCounter,
}

impl Metrics {
    pub fn new() -> Result<Self> {
        let connections =
            register_int_gauge!("mollysocket_connections", "Connections to Signal server")?;
        let reconnections =
            register_int_counter!("mollysocket_reconnections", "Reconnections since the start")?;
        let messages =
            register_int_counter!("mollysocket_messages", "Messages received from Signal")?;
        let pushs = register_int_counter!(
            "mollysocket_pushs",
            "Push messages sent to UnifiedPush endpoint"
        )?;

        Ok(Self {
            connections,
            reconnections,
            messages,
            pushs,
        })
    }
}

pub trait MountMetrics {
    fn mount_metrics<'a, B>(self, base: B, metrics: &Metrics) -> Self
    where
        B: TryInto<Origin<'a>> + Clone + Display,
        B::Error: Display;
}

impl MountMetrics for Rocket<Build> {
    fn mount_metrics<'a, B>(self, base: B, metrics: &Metrics) -> Self
    where
        B: TryInto<Origin<'a>> + Clone + Display,
        B::Error: Display,
    {
        let prometheus = PrometheusMetrics::new();
        let prom_registry = prometheus.registry();
        prom_registry
            .register(Box::new(metrics.connections.clone()))
            .unwrap();
        prom_registry
            .register(Box::new(metrics.reconnections.clone()))
            .unwrap();
        prom_registry
            .register(Box::new(metrics.messages.clone()))
            .unwrap();
        prom_registry
            .register(Box::new(metrics.pushs.clone()))
            .unwrap();

        self.attach(prometheus.clone()).mount(base, prometheus)
    }
}
