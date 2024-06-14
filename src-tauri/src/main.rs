// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use character::Character;
use tauri::{Manager, Window};

mod character;
mod win32;

struct AppState {
    hwnd: i64,
    handle: i64,
    battle_info_addr: i64,
    encoding: String,
    started: bool,
    characters: HashMap<i32, Character>,
}

#[tauri::command]
fn get_window_handle(state: tauri::State<Arc<Mutex<AppState>>>, window: tauri::Window) {
    let hwnd = win32::window_at_cursor_point();
    println!("hwnd: {hwnd}");
    {
        let mut state = state.lock().unwrap();
        state.hwnd = hwnd;
        window.emit("hwnd_changed", format!("{hwnd}")).unwrap();
    }
}

#[tauri::command]
fn start_monitoring(
    state: tauri::State<Arc<Mutex<AppState>>>,
    addr: &str,
    encoding: &str,
    window: tauri::Window,
) -> bool {
    let hwnd;
    {
        hwnd = state.lock().unwrap().hwnd;
    }
    println!("start monitoring on window({hwnd}) at address(0x{addr}) with encoding({encoding})");

    let battle_info_addr;
    if let Ok(addr) = i64::from_str_radix(addr, 16) {
        battle_info_addr = addr;
    } else {
        println!("failed to parse addr to i64");
        return false;
    }

    let h = win32::open_process(hwnd);
    if h <= 0 {
        println!("failed to open process");
        return false;
    }
    {
        let mut state = state.lock().unwrap();
        state.handle = h;
        state.battle_info_addr = battle_info_addr;
        state.encoding = encoding.to_owned();

        state.started = true;
        window.emit("started_changed", state.started).unwrap();
    }

    true
}

#[tauri::command]
fn stop_monitoring(state: tauri::State<Arc<Mutex<AppState>>>, window: tauri::Window) {
    println!("stop monitoring");
    let h;
    {
        let mut state = state.lock().unwrap();
        h = state.handle;
        state.started = false;
        window.emit("started_changed", state.started).unwrap();
        state.characters.clear();
        window.emit("characters_changed", "{}").unwrap();
    }

    win32::close_handle(h);
}

fn background_thread(state: Arc<Mutex<AppState>>, window: Window) {
    std::thread::spawn(move || loop {
        let started;
        let h;
        let addr;
        let encoding;
        {
            let state = state.lock().unwrap();
            started = state.started;
            h = state.handle;
            addr = state.battle_info_addr;
            encoding = state.encoding.clone();
        }
        if started {
            // read memory
            let battle_info = win32::read_memory_as_string(h, addr, 1000, &encoding);
            // println!("{battle_info}");

            if battle_info.len() > 0 {
                let fields: Vec<&str> = battle_info.split('|').collect();
                let mut characters_info: Vec<String> = Vec::new();
                for chunk in fields.chunks(12) {
                    if chunk.len() == 12 {
                        let combined = chunk.join("|");
                        characters_info.push(combined);
                    }
                }
                let mut characters: HashMap<i32, Character> = HashMap::new();
                for (_, chunk) in characters_info.iter().enumerate() {
                    // println!("Chunk {}: {}", index + 1, chunk);
                    // now we get index|name|cuid|?|lv|hp|hp_max|mp|mp_max|cid?|?|?    (wihout the last "|")
                    let character = Character::from(chunk);
                    // println!("{:?}", character);
                    characters.insert(character.pos_in_grid, character);
                }
                {
                    let mut state = state.lock().unwrap();
                    if state.characters != characters {
                        state.characters = characters.clone();
                        if let Ok(json) = serde_json::to_string(&characters) {
                            window.emit("characters_changed", json).unwrap();
                        }
                    }
                }
            } else {
                // println!("not in battle");
                {
                    let mut state = state.lock().unwrap();
                    if !state.characters.is_empty() {
                        state.characters.clear();

                        window.emit("characters_changed", "{}").unwrap();
                    }
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
    });
}

fn main() {
    win32::raise_privilege();

    let state = AppState {
        hwnd: 0,
        handle: 0,
        battle_info_addr: 0,
        encoding: String::new(),
        started: false,
        characters: HashMap::new(),
    };

    let state = Arc::new(Mutex::new(state));
    let thread_state = state.clone();

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            background_thread(thread_state, window);
            Ok(())
        })
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_window_handle,
            start_monitoring,
            stop_monitoring
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
