use opentelemetry::{
    global,
    sdk::trace::TracerProvider,
    trace::{Tracer, TracerProvider as _},
};

fn main() {
    let provider = TracerProvider::builder()
        .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
        .build();
    let tracer = provider.tracer("readme_example");

    tracer.in_span("doing the work", |cx| {
        println!("The work is being done now! {:#?}", cx);
    });

    global::shutdown_tracer_provider();
}
