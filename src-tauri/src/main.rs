// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{Menu, CustomMenuItem};

fn main() {
  let menu = Menu::new()
    .add_item(CustomMenuItem::new("file", "File"));
  tauri::Builder::default()
    .menu(menu)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
