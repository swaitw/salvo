use opentelemetry::{global, trace::TracerProvider as _, KeyValue};
use opentelemetry_sdk::{
    propagation::TraceContextPropagator, runtime, trace::TracerProvider, Resource,
};
use salvo::otel::{Metrics, Tracing};
use salvo::prelude::*;

mod exporter;
use exporter::Exporter;

fn init_tracer_provider() -> TracerProvider {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()
        .expect("failed to create exporter");
    opentelemetry_sdk::trace::TracerProvider::builder()
        .with_resource(Resource::new(vec![KeyValue::new(
            "service.name",
            "server2",
        )]))
        .with_batch_exporter(exporter, runtime::Tokio)
        .build()
}

#[handler]
async fn index(req: &mut Request) -> String {
    format!(
        "Body: {}",
        std::str::from_utf8(req.payload().await.unwrap()).unwrap()
    )
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let tracer = init_tracer_provider().tracer("app");
    let router = Router::new()
        .hoop(Metrics::new())
        .hoop(Tracing::new(tracer))
        .push(Router::with_path("api2").get(index))
        .push(Router::with_path("metrics").get(Exporter::new()));
    let acceptor = TcpListener::new("0.0.0.0:5801").bind().await;
    Server::new(acceptor).serve(router).await;
}
