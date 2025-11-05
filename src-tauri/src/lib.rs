use tauri::{async_runtime::Mutex, Manager};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub struct MyType {}

impl MyType {
    pub fn new() -> MyType {
        println!("Create MyType.");
        MyType {}
    }
}

impl Drop for MyType {
    fn drop(&mut self) {
        println!("Drop MyType.");
    }
}

pub struct State {
    pub my_type: MyType,
}

impl State {
    pub fn new() -> State {
        State {
            my_type: MyType::new(),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(move |app| {
            app.manage(Mutex::new(State::new()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
