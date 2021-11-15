use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubCommit {
    pub sha: String,
    pub node_id: String,
    pub commit: Commit,
    pub url: String,
    pub html_url: String,
    pub comments_url: String,
    pub author: Author,
    pub committer: Committer,
    pub parents: Vec<Parent>,
    pub stats: Stats,
    pub files: Vec<File>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    pub author: CommitAuthor,
    pub committer: CommitCommitter,
    pub message: String,
    pub tree: Tree,
    pub url: String,
    pub comment_count: i64,
    pub verification: Verification,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitAuthor {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitCommitter {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tree {
    pub sha: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Verification {
    pub verified: bool,
    pub reason: String,
    pub signature: Option<String>,
    pub payload: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Committer {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parent {
    pub sha: String,
    pub url: String,
    pub html_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub total: i64,
    pub additions: i64,
    pub deletions: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub sha: String,
    pub filename: String,
    pub status: String,
    pub additions: i64,
    pub deletions: i64,
    pub changes: i64,
    pub blob_url: String,
    pub raw_url: String,
    pub contents_url: String,
    pub patch: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubCompare {
    pub url: String,
    pub html_url: String,
    pub permalink_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub base_commit: BaseCommit,
    pub merge_base_commit: BaseCommit,
    pub status: String,
    pub ahead_by: i64,
    pub behind_by: i64,
    pub total_commits: i64,
    pub commits: Vec<BaseCommit>,
    pub files: Vec<File>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseCommit {
    pub sha: String,
    pub node_id: String,
    pub commit: Commit,
    pub url: String,
    pub html_url: String,
    pub comments_url: String,
    pub author: Author,
    pub committer: Committer,
    pub parents: Vec<Parent>,
}
