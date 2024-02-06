extern crate hldemo;

use demo_doer::demo_to_pointfile::demo_to_pointfile_cli;

mod demo_doer;
mod types;
mod utils;
mod writer;

fn main() {
    demo_to_pointfile_cli();
}
