//! BankFlow Tactical Analyzer - Rust Core Library
//!
//! Digital forensics tool for bank transaction and IP log correlation.

pub mod commands;
pub mod core;
pub mod models;
pub mod state;

use state::AppState;
use tauri::Manager;

/// Initialize and run the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState::new())
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::file_ops::load_file,
            commands::file_ops::load_ip_file,
            commands::file_ops::get_load_status,
            commands::file_ops::clear_files,
            commands::analysis::run_analysis,
            commands::file_ops::export_excel,
            commands::whois::query_whois,
            commands::whois::query_whois_batch,
            commands::system::check_update,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
