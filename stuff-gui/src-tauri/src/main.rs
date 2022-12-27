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
fn areas(store_lock: tauri::State<StoreSync>) -> Vec<stuff::Area> {
    if let Ok(store) = store_lock.lock() {
        store.state().areas()
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
    area_id: Option<&str>,
    store_lock: tauri::State<StoreSync>,
) {
    if let Ok(mut store) = store_lock.lock() {
        store.add_task(
            title,
            description,
            project_id.and_then(|pid| uuid::Uuid::parse_str(pid).ok()),
            project_heading_id.and_then(|pid| uuid::Uuid::parse_str(pid).ok()),
            area_id.and_then(|aid| uuid::Uuid::parse_str(aid).ok()),
        )
    } else {
        panic!("Failed to read the store! 😱");
    }
}

#[tauri::command]
fn move_task_to_position(task_id: &str, position: usize, store_lock: tauri::State<StoreSync>) {
    if let Ok(task_id) = uuid::Uuid::parse_str(task_id) {
        if let Ok(mut store) = store_lock.lock() {
            store.move_task_to_position(&task_id, position)
        } else {
            panic!("Failed to read the store! 😱");
        }
    }
}

#[tauri::command]
fn update_task_title(task_id: &str, title: &str, store_lock: tauri::State<StoreSync>) {
    if let Ok(task_id) = uuid::Uuid::parse_str(task_id) {
        if let Ok(mut store) = store_lock.lock() {
            store.update_task_title(&task_id, title)
        } else {
            panic!("Failed to read the store! 😱");
        }
    }
}

#[tauri::command]
fn update_task_description(task_id: &str, description: &str, store_lock: tauri::State<StoreSync>) {
    if let Ok(task_id) = uuid::Uuid::parse_str(task_id) {
        if let Ok(mut store) = store_lock.lock() {
            store.update_task_description(
                &task_id,
                if description.is_empty() {
                    None
                } else {
                    Some(description.to_string())
                },
            );
        } else {
            panic!("Failed to read the store! 😱");
        }
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
fn update_task_defer_date(
    task_id: &str,
    defer_date: Option<&str>,
    store_lock: tauri::State<StoreSync>,
) {
    if let Ok(task_id) = uuid::Uuid::parse_str(task_id) {
        if let Ok(mut store) = store_lock.lock() {
            store.update_task_defer_date(
                &task_id,
                defer_date.and_then(|t| chrono::NaiveDate::parse_from_str(t, "%Y-%m-%d").ok()),
            );
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
fn move_task_to_project_heading(
    task_id: &str,
    project_heading_id: &str,
    store_lock: tauri::State<StoreSync>,
) {
    if let (Ok(task_id), Ok(project_heading_id)) = (
        uuid::Uuid::parse_str(task_id),
        uuid::Uuid::parse_str(project_heading_id),
    ) {
        if let Ok(mut store) = store_lock.lock() {
            store.move_task_to_project_heading(&task_id, &project_heading_id);
        } else {
            panic!("Failed to read the store! 😱");
        }
    }
}

#[tauri::command]
fn clear_task_project_heading(task_id: &str, store_lock: tauri::State<StoreSync>) {
    if let Ok(task_id) = uuid::Uuid::parse_str(task_id) {
        if let Ok(mut store) = store_lock.lock() {
            store.clear_task_project_heading(&task_id);
        } else {
            panic!("Failed to read the store! 😱");
        }
    }
}

#[tauri::command]
fn create_project(
    name: &str,
    area_id: Option<&str>,
    store_lock: tauri::State<StoreSync>,
) -> Option<stuff::Project> {
    if let Ok(mut store) = store_lock.lock() {
        store
            .create_project(name, area_id.and_then(|t| uuid::Uuid::parse_str(t).ok()))
            .map(|p| p.clone())
    } else {
        unreachable!("Failed to read the store! 😱");
    }
}

#[tauri::command]
fn rename_project(project_id: &str, name: &str, store_lock: tauri::State<StoreSync>) {
    if let Ok(project_id) = uuid::Uuid::parse_str(project_id) {
        if let Ok(mut store) = store_lock.lock() {
            store.rename_project(&project_id, name);
        } else {
            unreachable!("Failed to read the store! 😱");
        }
    }
}

#[tauri::command]
fn move_project_to_position(
    project_id: &str,
    position: usize,
    store_lock: tauri::State<StoreSync>,
) {
    if let Ok(project_id) = uuid::Uuid::parse_str(project_id) {
        if let Ok(mut store) = store_lock.lock() {
            store.move_project_to_position(&project_id, position);
        } else {
            unreachable!("Failed to read the store! 😱");
        }
    }
}

#[tauri::command]
fn move_project_to_area(
    project_id: &str,
    area_id: Option<&str>,
    store_lock: tauri::State<StoreSync>,
) {
    if let Ok(project_id) = uuid::Uuid::parse_str(project_id) {
        if let Ok(mut store) = store_lock.lock() {
            store.move_project_to_area(
                &project_id,
                area_id.and_then(|t| uuid::Uuid::parse_str(t).ok()),
            );
        } else {
            unreachable!("Failed to read the store! 😱");
        }
    }
}

#[tauri::command]
fn add_project_heading(
    project_id: &str,
    name: &str,
    index: usize,
    store_lock: tauri::State<StoreSync>,
) {
    if let Ok(project_id) = uuid::Uuid::parse_str(project_id) {
        if let Ok(mut store) = store_lock.lock() {
            store.add_project_heading(&project_id, name, index);
        } else {
            panic!("Failed to read the store! 😱");
        }
    }
}

#[tauri::command]
fn create_area(name: &str, store_lock: tauri::State<StoreSync>) -> Option<stuff::Area> {
    if let Ok(mut store) = store_lock.lock() {
        store.create_area(name).map(|p| p.clone())
    } else {
        unreachable!("Failed to read the store! 😱");
    }
}

#[tauri::command]
fn rename_area(area_id: &str, name: &str, store_lock: tauri::State<StoreSync>) {
    if let Ok(area_id) = uuid::Uuid::parse_str(area_id) {
        if let Ok(mut store) = store_lock.lock() {
            store.rename_area(&area_id, name);
        } else {
            unreachable!("Failed to read the store! 😱");
        }
    }
}

#[tauri::command]
fn move_area_to_position(area_id: &str, position: usize, store_lock: tauri::State<StoreSync>) {
    if let Ok(area_id) = uuid::Uuid::parse_str(area_id) {
        if let Ok(mut store) = store_lock.lock() {
            store.move_area_to_position(&area_id, position);
        } else {
            unreachable!("Failed to read the store! 😱");
        }
    }
}

#[tauri::command]
fn read_config(client: tauri::State<stuff::Client>) -> Option<stuff::Config> {
    client.config()
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
            areas,
            add_task,
            move_task_to_position,
            update_task_title,
            update_task_description,
            mark_task_as_complete,
            mark_task_as_incomplete,
            move_task_to_inbox,
            update_task_defer_date,
            move_task_to_project,
            create_project,
            rename_project,
            move_project_to_position,
            move_project_to_area,
            add_project_heading,
            move_task_to_project_heading,
            clear_task_project_heading,
            create_area,
            rename_area,
            move_area_to_position,
            read_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
