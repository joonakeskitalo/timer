use uuid::Uuid;
use tauri::{ LogicalSize,Size };

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn create_window(app: tauri::AppHandle) {
  let id = Uuid::new_v4();
  let webview_window = tauri::WebviewWindowBuilder::new(&app, id, tauri::WebviewUrl::App("index.html".into()))
    .title("Timer")
    .build()
    .unwrap();

    webview_window.set_size(Size::Logical(LogicalSize { width: 480.0, height: 230.0})).unwrap()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![create_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
