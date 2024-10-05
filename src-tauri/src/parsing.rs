
mod parsers;
mod utils;
use std::io::BufReader;
use std::fs;
use parsers::Parser;
use serde_json;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime
};
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("parsing")
        .invoke_handler(tauri::generate_handler![test, parse_header])
        .build()
}
#[tauri::command]
fn test() -> String {
    "Hey plugins work".into()
}
#[tauri::command]
fn parse_header(path: String) -> String {
    let mut reader = BufReader::new(fs::File::open(path).unwrap());
    let header = parsers::DemoHeader::parse(
        (&mut reader)
    );
    match header {
        Ok(h) => serde_json::to_string(&h).unwrap(),
        Err(e) => "no".into()
    }
}