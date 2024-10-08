// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod parsing;
use tauri::{Menu, CustomMenuItem, Submenu};
use rfd::FileDialog;
use std::fs::{self};
use std::path::Path;


#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String
}
fn main() {
  let menu = Menu::new()
    .add_submenu(Submenu::new("File", Menu::new()
      .add_item(CustomMenuItem::new("open", "Open Gameinfo")))
    )
    .add_item(CustomMenuItem::new("edit", "Edit"))
    .add_item(CustomMenuItem::new("tools", "Tools"))
    .add_item(CustomMenuItem::new("view", "View"))
    .add_item(CustomMenuItem::new("help", "Help"));
  tauri::Builder::default()
    .menu(menu)
    .plugin(parsing::init())
    .on_menu_event(|event| {
      let _ = event.window().emit(("frontend_".to_owned() + event.menu_item_id().into()).as_str(), Payload {message: "none".to_owned()});
    })
    .invoke_handler(tauri::generate_handler![getfilepath, getfilesinpath, getbytesfromfile])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
#[tauri::command]
fn getfilepath() -> String {
  
  let answer = FileDialog::new()
    .add_filter("Gameinfo", &["txt"])
    .set_directory("/")
    .pick_file();
  let out = match &answer {
    Some(a) => a.as_path().to_str().expect("FU"),
    None => "nope"
  } ;
  return String::from(out);
}
#[tauri::command]
fn getfilesinpath(path: String) -> String {
  println!("{}", path);
  if !Path::new(&path).exists() {
    return String::from("nope");
  }
  let paths = fs::read_dir(path).unwrap();
  
  let mut out = String::from("");
  for path in paths {
    out += &(path.unwrap().path().display().to_string() + &String::from(","));
  }
  println!("{}",out);
  return out;
}
#[tauri::command]
fn getbytesfromfile(path: String) -> Vec<u8> {
  if !Path::new(&path).exists() {
    println!("{}", &path);
    return Vec::new();
  }
  let bytes = fs::read(path);
  let result = bytes.ok().unwrap();
  println!("{}", String::from_utf8_lossy(&result));
  return result
}