use reqwest::Client;
use reqwest_cookie_store::CookieStoreMutex;
use std::sync::{Arc, Mutex};

#[derive(serde::Deserialize)]
pub enum RequestType {
    GET,
    POST,
}

#[derive(PartialEq)]
pub enum AppPath {
    CONFIG,
    DATA,
}

// 管理Tauri状态对象
#[derive(Clone)]
pub struct AppState {
    pub client: Arc<Mutex<Client>>,
    pub cookie_store: Arc<CookieStoreMutex>,
    pub is_login: Arc<Mutex<bool>>,
    pub app_data_path: String,
}
