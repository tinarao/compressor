// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod zip;

#[tauri::command]
fn compress(files: Vec<String>, output_name: String) -> String {
    let result = match zip::compress(files, output_name) {
        Ok(()) => String::from("Ok!"),
        Err(_err) => String::from("Error!"),
    };

    result
}

#[tauri::command]
fn unzip(archive_path: String, target_dir: String) -> String {
    let result = match zip::unzip(archive_path, target_dir) {
        Ok(()) => String::from("Ok!"),
        Err(_err) => String::from("Error!"),
    };

    result
}

#[tauri::command]
fn contents(filepath: String) -> Vec<String> {
    match zip::get_contents(filepath) {
        Ok(v) => v,
        Err(_err) => Vec::new(),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![compress, unzip, contents])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
