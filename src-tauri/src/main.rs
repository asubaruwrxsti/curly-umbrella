mod api;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![api::load_collection, api::send_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
