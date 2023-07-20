#[allow(dead_code)]
pub fn convert_to_unsigned(bytes: Vec<u8>) -> Result<usize, ()> {
    match bytes.len() {
        1 => Ok(u8::from_be_bytes(bytes.try_into().unwrap()) as usize),
        2 => Ok(u16::from_be_bytes(bytes.try_into().unwrap()) as usize),
        4 => Ok(u32::from_be_bytes(bytes.try_into().unwrap()) as usize),
        8 => Ok(u64::from_be_bytes(bytes.try_into().unwrap()) as usize),
        16 => Ok(u128::from_be_bytes(bytes.try_into().unwrap()) as usize),
        _ => Err(()),
    }
}
#[allow(dead_code)]
pub fn convert_to_signed(bytes: Vec<u8>) -> Result<usize, ()> {
    match bytes.len() {
        1 => Ok(i8::from_be_bytes(bytes.try_into().unwrap()) as usize),
        2 => Ok(i16::from_be_bytes(bytes.try_into().unwrap()) as usize),
        4 => Ok(i32::from_be_bytes(bytes.try_into().unwrap()) as usize),
        8 => Ok(i64::from_be_bytes(bytes.try_into().unwrap()) as usize),
        16 => Ok(i128::from_be_bytes(bytes.try_into().unwrap()) as usize),
        _ => Err(()),
    }
}

use std::path::PathBuf;
use tauri::{api::path::resolve_path, api::path::BaseDirectory, Env};
pub fn get_default_dir(directory: BaseDirectory) -> Result<PathBuf, ()> {
    let context: tauri::Context<tauri::utils::assets::EmbeddedAssets> = tauri::generate_context!();
    Ok(resolve_path(
        context.config(),
        context.package_info(),
        &Env::default(),
        "",
        Some(directory),
    )
    .map_err(|_err| ())?)
}
