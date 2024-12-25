use common::Error;

fn validate_name(_name: &str) -> Result<(), Error> {
    Err(Error::InvalidName("Tbt".into()))
}

#[tauri::command]
fn greet(name: &str) -> Result<String, Error> {
    validate_name(name)?;

    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
