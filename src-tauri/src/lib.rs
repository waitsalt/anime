pub mod common;
pub mod module;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        // .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![
            module::config::config_get,
            module::config::config_set,
            module::bangumi::bangumi_access_token_check,
            module::bangumi::bangumi_get_me,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
