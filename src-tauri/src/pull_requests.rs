use crate::{GITHUB_CLIENT, OWNER, REPO};

#[derive(serde::Serialize)]
pub struct PullRequest {
    pub title: String,
    pub last_commit: String,
    pub last_review: String,
}

#[tauri::command]
pub async fn get_prs() -> Result<Vec<PullRequest>, ()> {
    let page = GITHUB_CLIENT
        .pulls(OWNER.to_string(), REPO.to_string())
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
