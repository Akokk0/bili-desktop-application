// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
use app::set_up_func;
use command::*;

fn main() {
    tauri::Builder::default()
        .setup(set_up_func)
        .invoke_handler(tauri::generate_handler![
            save_cookies,
            request,
            is_login,
            logout,
            check_cookies_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
