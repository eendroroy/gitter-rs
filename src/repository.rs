use crate::repository_helper::get_repo_status;
use std::cmp::max;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::task::JoinSet;

#[derive(Debug, Default, Clone)]
pub struct Status {
    pub absolute_path: String,
    pub path: String,
    pub name: String,
    pub branch: String,
    pub author_name: String,
    pub author_email: String,
    pub relative_time: String,
    pub absolute_time: String,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct StatusLengths {
    pub path: usize,
    pub name: usize,
    pub branch: usize,
    pub author_name: usize,
    pub author_email: usize,
    pub relative_time: usize,
    pub absolute_time: usize,
}

#[derive(Debug, Clone)]
pub struct Repositories {
    pub statuses: Vec<Status>,
    pub lengths: StatusLengths,
}

impl Repositories {
    pub async fn new(repositories: Vec<PathBuf>, path: &String) -> Self {
        let base_path = Arc::new(path.clone());

        let mut tasks = JoinSet::new();

        for repo in repositories {
            let base_path = Arc::clone(&base_path);
            tasks.spawn_blocking(move || {
                get_repo_status(repo.to_str().expect("Invalid UTF-8 path"), &**base_path)
            });
        }

        let mut statuses: Vec<Status> = Vec::new();

        while let Some(result) = tasks.join_next().await {
            match result {
                Ok(status) => statuses.push(status),
                Err(e) => eprintln!("Task failed: {e}"),
            }
        }

        statuses.sort_by(|a, b| a.name.cmp(&b.name));
        let repos: Self = Self {
            statuses,
            lengths: StatusLengths::default(),
        };
        repos
    }

    pub fn compute_lengths(&mut self) {
        self.statuses.iter().for_each(|s| {
            self.lengths.path = max(self.lengths.path, s.path.len());
            self.lengths.name = max(self.lengths.name, s.name.len());
            self.lengths.branch = max(self.lengths.branch, s.branch.len());
            self.lengths.author_name = max(self.lengths.author_name, s.author_name.len());
            self.lengths.author_email = max(self.lengths.author_email, s.author_email.len());
            self.lengths.relative_time = max(self.lengths.relative_time, s.relative_time.len());
            self.lengths.absolute_time = max(self.lengths.absolute_time, s.absolute_time.len());
        });
    }
}
