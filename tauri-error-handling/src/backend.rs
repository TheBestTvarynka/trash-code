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

pub async fn greet(name: &str) -> Option<()> {
    let args = to_value(&GreetArgs { name }).expect("GreetArgs to JsValue should not fail");

    let js_value = invoke("greet", args).await.expect("should not fail");

    let result: Option<()> = from_value(js_value).expect("deserialization should not fail");
    info!("Result: {:?}", result);

    result
}
