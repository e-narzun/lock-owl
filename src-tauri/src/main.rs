// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path;
use tauri::WindowEvent;
mod lowl_encrypt;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command(async)]
fn tauri_encrypt_file(file_path: &str, overwrite: bool) -> Result<lowl_encrypt::FileDisplay, ()> {
    Ok(
        match lowl_encrypt::encrypt_file(
            path::Path::new(file_path),
            overwrite,
            1 as u16,
            "undefined",
        ) {
            Result::Ok(file_name) => file_name,
            Result::Err(err) => {
                dbg!(err);
                return Err(());
            }
        },
    )
}

#[tauri::command(async)]
fn tauri_decrypt_file(
    file_path: &str,
    overwrite: bool,
    password: &str,
) -> Result<lowl_encrypt::FileDisplay, ()> {
    Ok(
        match lowl_encrypt::decrypt_file(path::Path::new(file_path), overwrite, password) {
            Result::Ok(filename) => {
                println!("{} successful decrypted", &file_path);
                filename
            }
            Result::Err(err) => {
                dbg!(err);
                return Err(());
            }
        },
    )
}

#[tauri::command]
fn create_new_user(username: &str, password: &str, overwrite: bool) -> Result<(), ()> {
    match lowl_encrypt::generate_new_keys(overwrite, password) {
        Result::Ok(_) => return Ok(()),
        Result::Err(err) => {
            dbg!(err);
            return Err(());
        }
    }
}
#[tauri::command]
fn check_private_key() -> Result<(), ()> {
    let path = lowl_encrypt::utility::get_default_dir(tauri::api::path::BaseDirectory::AppConfig)
        .unwrap()
        .join("private.key");
    if path.is_file() {
        return Ok(());
    } else {
        return Err(());
    }
}

fn main() {
    tauri::Builder::default()
        .on_window_event(|e| {
            if let WindowEvent::Resized(_) = e.event() {
                std::thread::sleep(std::time::Duration::from_nanos(1));
            }
        })
        .invoke_handler(tauri::generate_handler![
            tauri_encrypt_file,
            tauri_decrypt_file,
            create_new_user,
            check_private_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
