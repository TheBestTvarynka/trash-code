use common::Error;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{to_value, from_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

pub async fn greet(name: &str) -> Result<String, Error> {
    let args = to_value(&GreetArgs { name }).expect("GreetArgs to JsValue should not fail");

    match invoke("greet", args).await {
        Ok(msg) => Ok(from_value(msg).unwrap()),
        Err(err) => Err(from_value(err).unwrap()),
    }
}
