#[allow(dead_code)]
mod common;
mod utils;
#[allow(unused_imports)]
use common::BIG_TEXT;
use utils::devtracing;

fn main() {
    println!("Hello, world!");
    devtracing::builder();
    debug!("{}", BIG_TEXT);
}
