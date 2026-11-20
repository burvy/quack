#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("starting quack");
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
