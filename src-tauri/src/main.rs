#![feature(iter_collect_into)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod pull_requests;

use crate::pull_requests::get_prs;
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
    static ref OWNER: String = env::var("OWNER").unwrap();
    static ref REPO: String = env::var("REPO").unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_prs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
