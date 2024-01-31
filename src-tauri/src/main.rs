// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{api::{file::read_binary, path::{resolve_path, BaseDirectory}}, http::ResponseBuilder, Manager};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .register_uri_scheme_protocol("module", handle_module_request)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn handle_module_request(app: &tauri::AppHandle, req: &tauri::http::Request) -> Result<tauri::http::Response, Box<dyn std::error::Error>> {
    // get the path from the request
    let url = req.uri().parse::<url::Url>()?;
    let path = match url.path() {
        "/" => "index.html",
        path => path,
    };

    let resource_path = resolve_path(
        &app.config(),
        app.package_info(),
        &app.env(),
        format!("modules/module/{}", path),
        Some(BaseDirectory::Resource)
    )?;

    let content_type = match resource_path.extension() {
        Some(ext) => match ext.to_str() {
            Some("html") => "text/html",
            Some("js") => "text/javascript",
            Some("css") => "text/css",
            Some("png") => "image/png",
            Some("jpg") => "image/jpeg",
            Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            Some("svg") => "image/svg+xml",
            Some("ico") => "image/x-icon",
            Some("json") => "application/json",
            Some("woff") => "font/woff",
            Some("woff2") => "font/woff2",
            Some("ttf") => "font/ttf",
            Some("otf") => "font/otf",
            Some("eot") => "application/vnd.ms-fontobject",
            Some("sfnt") => "application/font-sfnt",
            Some("xml") => "text/xml",
            Some("pdf") => "application/pdf",
            Some("zip") => "application/zip",
            Some("gz") => "application/gzip",
            Some("tar") => "application/x-tar",
            Some("mp3") => "audio/mpeg",
            Some("wav") => "audio/wav",
            Some("ogg") => "audio/ogg",
            Some("mp4") => "video/mp4",
            Some("webm") => "video/webm",
            Some("webp") => "image/webp",
            Some("txt") => "text/plain",
            _ => "application/octet-stream",
        },
        None => "application/octet-stream",
    };

    if resource_path.exists() {
        ResponseBuilder::new()
            .status(200)
            .header("Content-Type", content_type)
            .header("Access-Control-Allow-Origin", "*")
            .body(read_binary(resource_path.as_path()).unwrap())
    } else {
        ResponseBuilder::new()
            .status(404)
            .body("File not found".as_bytes().to_vec())
    }
}