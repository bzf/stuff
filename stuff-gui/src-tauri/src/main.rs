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
fn project_headings(store_lock: tauri::State<StoreSync>) -> Vec<stuff::ProjectHeading> {
    if let Ok(store) = store_lock.lock() {
        store.state().project_headings()
    } else {
        panic!("Failed to read the store! 😱");
    }
}

#[tauri::command]
fn add_task(
    title: &str,
    description: Option<String>,
    project_id: Option<&str>,
    project_heading_id: Option<&str>,
    store_lock: tauri::State<StoreSync>,
) {
    if let Ok(mut store) = store_lock.lock() {
        store.add_task(
            title,
            description,
            project_id.and_then(|pid| uuid::Uuid::parse_str(pid).ok()),
            project_heading_id.and_then(|pid| uuid::Uuid::parse_str(pid).ok()),
        )
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

#[tauri::command]
fn move_task_to_inbox(task_id: &str, store_lock: tauri::State<StoreSync>) {
    if let Ok(task_id) = uuid::Uuid::parse_str(task_id) {
        if let Ok(mut store) = store_lock.lock() {
            store.move_task_to_inbox(&task_id);
        } else {
            panic!("Failed to read the store! 😱");
        }
    }
}

#[tauri::command]
fn move_task_to_project(task_id: &str, project_id: &str, store_lock: tauri::State<StoreSync>) {
    if let (Ok(task_id), Ok(project_id)) = (
        uuid::Uuid::parse_str(task_id),
        uuid::Uuid::parse_str(project_id),
    ) {
        if let Ok(mut store) = store_lock.lock() {
            store.move_task_to_project(&task_id, &project_id);
        } else {
            panic!("Failed to read the store! 😱");
        }
    }
}

#[tauri::command]
fn create_project(name: &str, store_lock: tauri::State<StoreSync>) {
    if let Ok(mut store) = store_lock.lock() {
        store.create_project(name);
    } else {
        panic!("Failed to read the store! 😱");
    }
}

#[tauri::command]
fn add_project_heading(project_id: &str, name: &str, store_lock: tauri::State<StoreSync>) {
    if let Ok(project_id) = uuid::Uuid::parse_str(project_id) {
        if let Ok(mut store) = store_lock.lock() {
            store.add_project_heading(&project_id, name);
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
            project_headings,
            add_task,
            mark_task_as_complete,
            mark_task_as_incomplete,
            move_task_to_inbox,
            move_task_to_project,
            create_project,
            add_project_heading,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
