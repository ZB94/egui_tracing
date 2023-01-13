#[macro_use]
extern crate tracing;

use tracing::Span;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

fn main() {
    let (layer, mut widget) = egui_tracing::layer::EguiLayer::new(1000);

    let layer = layer.with_filter(tracing_subscriber::EnvFilter::from_default_env());

    tracing_subscriber::registry().with(layer).init();

    warp_log();

    widget.ui();
}

#[instrument(name = "log_fn", fields(c))]
fn log(a: &str, b: i32) {
    let span = Span::current();
    info!(with = "start", "before");
    span.record("c", "field c");
    warn!(with = "end", "after");
}

#[instrument]
fn warp_log() {
    log("field a", 2);
}
