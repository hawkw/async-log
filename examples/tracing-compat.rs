use async_log::{instrument, span};
use log::info;

fn setup_logger() {
    use tracing_subscriber::{filter::LevelFilter, fmt};
    fmt::Subscriber::builder()
        .with_max_level(LevelFilter::TRACE)
        .init();
}

fn main() {
    setup_logger();

    span!("level {}", 1, {
        let x = "beep";
        info!("look at this value: {}", x);

        span!("level {}", 2, {
            inner("boop");
        })
    })
}

#[instrument]
fn inner(y: &str) {
    info!("another nice value: {}", y);
}
