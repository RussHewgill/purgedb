use tracing_log::LogTracer;
use tracing_subscriber::{prelude::*, registry::Registry, EnvFilter};

use anyhow::{anyhow, bail, ensure, Context, Result};
use tracing::{debug, error, info, trace, warn};

pub fn init_logs() {
    let _ = std::fs::rename("output.log", "output_prev.log");
    let _ = std::fs::remove_file("output.log");

    let trace_file =
        tracing_appender::rolling::never(".", "output.log").with_max_level(tracing::Level::TRACE);

    // LogTracer::init().unwrap();

    let file_layer = tracing_subscriber::fmt::Layer::new()
        .with_writer(trace_file)
        .with_file(true)
        .with_ansi(false)
        .with_line_number(true)
        .with_target(true)
        .with_level(true)
        .compact()
        .with_filter(tracing_subscriber::filter::EnvFilter::new(
            "info,purgedb=trace,eframe=warn,wgpu=warn",
        ));

    let stderr_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .without_time()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .compact()
        .with_filter(tracing_subscriber::filter::EnvFilter::new(
            // "info,purgedb=debug,eframe=warn,wgpu=warn",
            "warn,purgedb=debug,eframe=warn,wgpu=warn",
        ));

    let subscriber = tracing_subscriber::registry()
        .with(file_layer)
        .with(stderr_layer)
        .try_init()
        .unwrap();
}
