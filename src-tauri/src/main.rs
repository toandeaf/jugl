#![feature(iter_collect_into)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;
use lazy_static::lazy_static;
use octocrab::Octocrab;
use std::env;

lazy_static! {
    static ref GITHUB_CLIENT: Octocrab = {
        dotenv().ok();
        Octocrab::builder()
            .basic_auth(
                env::var("GITHUB_USERNAME").unwrap(),
                env::var("GITHUB_PASSWORD").unwrap(),
            )
            .build()
            .unwrap_or(Octocrab::default())
    };
}

fn main() {
    dotenv().ok();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_prs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(serde::Serialize)]
struct PullRequest {
    title: String,
    last_commit: String,
    last_review: String,
}

#[tauri::command]
async fn get_prs() -> Result<Vec<PullRequest>, ()> {
    // Returns the first page of all issues.
    let page = GITHUB_CLIENT
        .pulls("toandeaf", "jugl")
        .list()
        .per_page(10)
        .send()
        .await
        .unwrap();

    let mut prs_vec: Vec<PullRequest> = Vec::with_capacity(page.items.len());

    page.items
        .iter()
        .map(|pr| PullRequest {
            title: pr.title.clone().unwrap(),
            last_review: String::from("There"),
            last_commit: String::from("Before"),
        })
        .collect_into(&mut prs_vec);

    Ok(prs_vec)
}
