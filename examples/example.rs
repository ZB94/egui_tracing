#[macro_use]
extern crate tracing;

use eframe::{App, Frame};
use egui::{CentralPanel, Context};
use egui_tracing::widget::EguiLog;
use tracing::Span;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

fn main() {
    let (layer, widget) = egui_tracing::layer::EguiLayer::new(10);

    let layer = layer.with_filter(
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("example=trace,warn")),
    );

    tracing_subscriber::registry().with(layer).init();

    warp_log();

    eframe::run_native(
        "example",
        Default::default(),
        Box::new(|_| Box::new(Example(widget))),
    );
}

#[instrument(name = "log_fn", fields(c))]
fn log(a: &str, b: i32) {
    let span = Span::current();
    info!(with = "start", "before");
    span.record("c", "field c");
    warn!(with = "end", "after");

    debug!("{}", "long ".repeat(100));

    error!("{}", "multi line\n".repeat(10));

    trace!("end");
}

#[instrument]
fn warp_log() {
    log("field a", 2);
}

pub struct Example(pub EguiLog);

impl App for Example {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            if ui.button("log").clicked() {
                warp_log();
                info!("clicked");
            }

            ui.add(&mut self.0);
        });
    }
}
