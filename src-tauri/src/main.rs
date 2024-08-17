// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Menu, CustomMenuItem, Submenu};
use rfd::FileDialog;


#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String
}
fn main() {
  let menu = Menu::new()
    .add_submenu(Submenu::new("File", Menu::new().add_item(CustomMenuItem::new("open", "Open"))));
  tauri::Builder::default()
    .menu(menu)
    .on_menu_event(|event| {
      event.window().emit(("frontend_".to_owned() + event.menu_item_id().into()).as_str(), Payload {message: "none".to_owned()});
    })
    .invoke_handler(tauri::generate_handler![getfilepath])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
#[tauri::command]
fn getfilepath() -> String {
  //println!("yep");
  let mut answer = FileDialog::new()
    .add_filter("Demo file", &["dem"])
    .set_directory("/")
    .pick_file();
  return answer.expect("nope").as_path().to_string_lossy().into();
  
}