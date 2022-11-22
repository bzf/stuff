#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn tasks(stuff_client: tauri::State<stuff::Client>) -> Vec<stuff::Task> {
    stuff_client.store().tasks()
}

#[tauri::command]
fn projects(stuff_client: tauri::State<stuff::Client>) -> Vec<stuff::Project> {
    stuff_client.store().projects()
}

#[tauri::command]
fn add_task(title: &str, stuff_client: tauri::State<stuff::Client>) {
    stuff_client.store().add_task(title)
}

#[tauri::command]
fn mark_task_as_complete(task_id: &str, stuff_client: tauri::State<stuff::Client>) {
    if let Ok(task_id) = uuid::Uuid::parse_str(task_id) {
        stuff_client.store().mark_task_as_complete(&task_id);
    }
}

#[tauri::command]
fn mark_task_as_incomplete(task_id: &str, stuff_client: tauri::State<stuff::Client>) {
    if let Ok(task_id) = uuid::Uuid::parse_str(task_id) {
        stuff_client.store().mark_task_as_incomplete(&task_id);
    }
}

fn main() {
    let client = stuff::Client::new("stuff-gui");

    if client.config().is_none() {
        client.initialize();
    }

    client.config().expect("Failed to load stuff configuration");

    tauri::Builder::default()
        .manage(client)
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
