mod app;
mod backend;

#[macro_use]
extern crate log;

use app::*;
use leptos::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
