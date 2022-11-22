#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};

type StoreSync = Arc<Mutex<stuff::Store>>;

#[tauri::command]
fn tasks(store_lock: tauri::State<StoreSync>) -> Vec<stuff::Task> {
    if let Ok(store) = store_lock.lock() {
        store.state().tasks()
    } else {
        panic!("Failed to read the store! 😱");
    }
}

#[tauri::command]
fn projects(store_lock: tauri::State<StoreSync>) -> Vec<stuff::Project> {
    if let Ok(store) = store_lock.lock() {
        store.state().projects()
    } else {
        panic!("Failed to read the store! 😱");
    }
}

#[tauri::command]
fn add_task(title: &str, store_lock: tauri::State<StoreSync>) {
    if let Ok(mut store) = store_lock.lock() {
        store.add_task(title)
    } else {
        panic!("Failed to read the store! 😱");
    }
}

#[tauri::command]
fn mark_task_as_complete(task_id: &str, store_lock: tauri::State<StoreSync>) {
    if let Ok(task_id) = uuid::Uuid::parse_str(task_id) {
        if let Ok(mut store) = store_lock.lock() {
            store.mark_task_as_complete(&task_id);
        } else {
            panic!("Failed to read the store! 😱");
        }
    }
}

#[tauri::command]
fn mark_task_as_incomplete(task_id: &str, store_lock: tauri::State<StoreSync>) {
    if let Ok(task_id) = uuid::Uuid::parse_str(task_id) {
        if let Ok(mut store) = store_lock.lock() {
            store.mark_task_as_incomplete(&task_id);
        } else {
            panic!("Failed to read the store! 😱");
        }
    }
}

fn main() {
    use tauri::Manager;

    let client = stuff::Client::new("stuff-gui");

    if client.config().is_none() {
        client.initialize();
    }

    client.config().expect("Failed to load stuff configuration");
    let (store, receiver) = client.store();

    tauri::Builder::default()
        .manage(client)
        .manage(Arc::new(Mutex::new(store)))
        .setup(|app| {
            let app_handle = app.app_handle();

            let _join_handle = std::thread::spawn(move || loop {
                if let Some(next_state) = receiver.recv().ok() {
                    app_handle.emit_all("next-stuff-state", next_state).unwrap();
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            tasks,
            projects,
            add_task,
            mark_task_as_complete,
            mark_task_as_incomplete,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
