
mod parsers;
mod utils;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime
};
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("parsing")
        .invoke_handler(tauri::generate_handler![test])
        .build()
}
#[tauri::command]
fn test() -> String {
    "Hey plugins work".into()
}