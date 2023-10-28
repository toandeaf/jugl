// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use octocrab::params;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_prs() -> Result<String, ()> {
    let octocrab = octocrab::instance();

    // Returns the first page of all issues.
    let page = octocrab
        .pulls("toandeaf", "jugl")
        .list()
        .head("master")
        .state(params::State::All)
        .per_page(100)
        .page(5u32)
        // Send the request
        .send()
        .await
        .unwrap();

    let name = octocrab
        .repos("toandeaf", "jugl")
        .get()
        .await
        .unwrap()
        .default_branch
        .unwrap_or(String::from("reeb"));

    println!("Value is {}", name);

    Ok(String::from("PR Test"))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_prs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
