use tracing::trace;

mod common;
use common::BIG_TEXT;

fn main() {
    println!("Hello, world!");
    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    trace!("{}", BIG_TEXT);
}
