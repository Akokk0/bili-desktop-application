use std::error::Error;
use std::fs;
use std::sync::{Arc, Mutex};

use reqwest::header::{HeaderMap, HeaderValue, ORIGIN, REFERER, USER_AGENT};
use reqwest::Client;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use tauri::Manager;

mod object;
pub use object::*;

// Creates default request header
pub fn create_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (HTML, like Gecko) Chrome/96.0.4664.93 Safari/537.36"));
    headers.insert(
        ORIGIN,
        HeaderValue::from_static("https://space.bilibili.com"),
    );
    headers.insert(
        REFERER,
        HeaderValue::from_static("https://www.bilibili.com"),
    );
    headers
}

// Load cookies
fn load_cookies(app_data_path: &String) -> Arc<CookieStoreMutex> {
    // Define cookies file path
    let cookies_file_path = format!("{}/auth/cookies.json", app_data_path);
    // Load cookies from file
    let cookie_store = 
        // Check if path is a file
        if std::path::Path::new(&cookies_file_path).is_file() {
            // Path is a file, reading file content and load cookies
            let file = fs::File::open(&cookies_file_path)
            .map(std::io::BufReader::new)
            .unwrap();
            // Return the loaded cookie_store
            CookieStore::load_json(file).unwrap()
        } else {
            // Path is not a file, returned a default cookie_store
            CookieStore::default()
        };
    // Wrap cookie_store as Arc<CookieStoreMutex<CookieStore>>
    let cookie_store = CookieStoreMutex::new(cookie_store);
    let cookie_store = Arc::new(cookie_store);
    // Print successful loading information
    {
        println!("load cookies");
        let store = cookie_store.lock().unwrap();
        for c in store.iter_any() {
            println!("{:?}", c)
        }
    }
    // Return the wrapped CookieStore
    return cookie_store;
}

// Get app path
fn get_app_path(path_type: AppPath, config: &tauri::Config) -> Option<String> {
    // Get the corresponding path based on the path type
    let path = match path_type {
        AppPath::CONFIG => tauri::api::path::app_config_dir(config).unwrap(),
        AppPath::DATA => tauri::api::path::app_data_dir(config).unwrap(),
    };
    // Return folder path
    Some(path.to_string_lossy().to_string())
}

// Check if file is empty
fn is_file_empty(path: &str) -> std::io::Result<bool> {
    // Check if path is a file
    if !std::path::Path::new(&path).is_file() { return Ok(true); }
    // Check if file is empty
    let metadata = fs::metadata(path)?;
    // Return result
    Ok(metadata.len() == 0)
}

// Tauri setup function(manage state object)
pub fn set_up_func(app: &mut tauri::App) -> Result<(), Box<dyn Error>> {
    // Get AppConfig Path
    let app_data_path = get_app_path(AppPath::DATA, &app.config()).unwrap();
    // Get CookieStore
    let cookie_store = load_cookies(&app_data_path);
    // Create reqwest Client
    let client = Client::builder()
        .cookie_provider(Arc::clone(&cookie_store))
        .default_headers(create_headers())
        .build()
        .unwrap();
    // Wrap client as Arc<Mutex<Client>>
    let client = Mutex::new(client);
    let client = Arc::new(client);
    // Get login status and wrap it as Arc<Mutex<bool>>
    let is_login =
        !is_file_empty(format!("{}/auth/cookies.json", &app_data_path).as_str()).unwrap();
    let is_login = Mutex::new(is_login);
    let is_login = Arc::new(is_login);
    // Manage state object
    app.manage(AppState {
        // Manage request client、CookieStore、config file path
        client,
        cookie_store,
        is_login,
        app_data_path,
    });
    // Return success
    Ok(())
}
