use std::{env, thread, time::Duration};

use crate::types::github::{Commit, GitHubCommit, GitHubCompare};

#[derive(Debug, thiserror::Error)]
pub enum CreateIssuesError {
    #[error("No Hash could be found.")]
    NoHashFound,
    #[error("Error from reqwest: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

pub async fn create_issues(url: String) -> Result<(), CreateIssuesError> {
    fn build_request_headers(mut request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        request = request.header(reqwest::header::USER_AGENT, "Discordeno Issue Creation Bot");
        request = request.header(
            reqwest::header::AUTHORIZATION,
            format!("token {}", env::var("GITHUB_ACCESS_TOKEN").unwrap()),
        );

        request
    }

    if let Ok(client) = reqwest::Client::builder().build() {
        let hash_part = url.split('/').last();
        if hash_part == None {
            return Err(CreateIssuesError::NoHashFound);
        }

        let mut commits: Vec<Commit> = vec![];

        // Check whether its a normal commit url or an url to a compare.
        if !hash_part.unwrap().contains("...") {
            // Its a normal commit URL
            // First get the issue
            let mut request = client.get(format!(
                "https://api.github.com/repos/discord/discord-api-docs/commits/{}",
                hash_part.unwrap()
            ));

            request = build_request_headers(request);
            let res = request.send().await;
            let commit_data = res.unwrap().json::<GitHubCommit>().await?;
            commits.push(commit_data.commit);

            // if let Ok(res) = request.send().await {
            //     let commit_data = res.json::<GitHubCommit>().await?;
            //     commits.push(commit_data.commit)
            // }
        } else {
            // Its a compare URL
            // First get the issue
            let mut request = client.get(format!(
                "https://api.github.com/repos/discord/discord-api-docs/compare/{}",
                hash_part.unwrap()
            ));

            request = build_request_headers(request);
            let res = request.send().await;
            let compare_data = res.unwrap().json::<GitHubCompare>().await?;
            commits = compare_data
                .commits
                .into_iter()
                .map(|cmp| cmp.commit)
                .collect();
        }

        let mut len = commits.len();

        for commit in commits {
            let url = match commit.url.split('/').last() {
                Some(hash) => format!(
                    "https://github.com/discord/discord-api-docs/commit/{}",
                    hash
                ),
                None => commit.url,
            };

            let mut request =
                client.post("https://api.github.com/repos/discordeno/discordeno/issues");

            request = build_request_headers(request);
            request = request.header(reqwest::header::ACCEPT, "application/vnd.github.v3+json");
            request
                .json(&GithubCreateIssue::new(commit.message, url))
                .send()
                .await?;

            len -= 1;
            if len != 0 {
                thread::sleep(Duration::from_secs(5));
            }
        }
    }

    Ok(())
}

#[derive(Debug, serde::Serialize)]
pub struct GithubCreateIssue {
    title: String,
    body: String,
    labels: Vec<String>,
}

impl GithubCreateIssue {
    fn new(commit_message: String, url: String) -> Self {
        Self {
            title: format!("[api-docs] {}", commit_message),
            body: format!(
                "A new commit was made into the api-docs repo.\n{}\n\nThis is a bot created issue.",
                url
            ),
            labels: vec!["api-docs-commits".into()],
        }
    }
}
