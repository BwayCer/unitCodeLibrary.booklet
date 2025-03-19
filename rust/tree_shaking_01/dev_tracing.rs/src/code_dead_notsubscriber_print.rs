use tracing::trace;

mod common;
use common::BIG_TEXT;

fn main() {
    println!("Hello, world!");
    trace!("{}", BIG_TEXT);
}
