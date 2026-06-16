use crate::repository::helper::{
    USER_EMAIL, USER_NAME, extract_config, get_absolute_time, get_bare, get_branch_count,
    get_commit_count, get_contributor_summary, get_current_branch, get_current_commit_hash,
    get_dirty, get_relative_path, get_relative_time, get_repo_name,
};
use git2::Repository;
use std::cmp::max;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::task::JoinSet;

#[derive(Debug, Default, Clone)]
pub struct ContributionSummary {
    pub author_count: usize,
    pub top_commit_count: usize,
    pub top_author_name: String,
    pub top_author_email: String,
}

#[derive(Debug, Default, Clone)]
pub struct Properties {
    pub absolute_path: String,
    pub relative_path: String,
    pub name: String,
    pub branch: String,
    pub branch_count: usize,
    pub commit_hash: String,
    pub commit_count: usize,
    pub author_name: String,
    pub author_email: String,
    pub relative_time: String,
    pub absolute_time: String,
    pub dirty: String,
    pub is_dirty: bool,
    pub bare: String,
    pub is_bare: bool,
    pub contribution_summary: ContributionSummary,
}

impl Properties {
    pub fn new(path: &str, base_path: &str) -> Option<Self> {
        let repository = Repository::open(path).ok()?;
        let config = repository.config().ok();

        let absolute_path = path.to_string();
        let relative_path = get_relative_path(path, base_path);
        let name = get_repo_name(path);
        let branch = get_current_branch(&repository);
        let branch_count = get_branch_count(&repository);
        let commit_hash = get_current_commit_hash(&repository);
        let commit_count = get_commit_count(&repository);
        let author_name = extract_config(&config, USER_NAME);
        let author_email = extract_config(&config, USER_EMAIL);
        let relative_time = get_relative_time(&repository);
        let absolute_time = get_absolute_time(&repository);
        let (dirty, is_dirty) = get_dirty(&repository);
        let (bare, is_bare) = get_bare(&repository);
        let contribution_summary = get_contributor_summary(&repository);

        Some(Self {
            absolute_path,
            relative_path,
            name,
            branch,
            branch_count,
            commit_hash,
            commit_count,
            author_name,
            author_email,
            relative_time,
            absolute_time,
            dirty,
            is_dirty,
            bare,
            is_bare,
            contribution_summary,
        })
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PropertyLengths {
    pub path: usize,
    pub name: usize,
    pub branch: usize,
    pub branch_count: usize,
    pub commit_count: usize,
    pub author_name: usize,
    pub author_email: usize,
    pub relative_time: usize,
    pub absolute_time: usize,
    pub bare: usize,
    pub cs_author_count: usize,
    pub cs_commit_count: usize,
    pub cs_top_commit_count: usize,
    pub cs_top_author_name: usize,
    pub cs_top_author_email: usize,
}

#[derive(Debug, Clone)]
pub struct Repositories {
    pub props: Vec<Properties>,
    pub lens: PropertyLengths,
}

impl Repositories {
    pub async fn new(repositories: Vec<PathBuf>, path: &str) -> Self {
        let base_path = Arc::new(path.to_owned());

        let mut tasks = JoinSet::new();

        for repo in repositories {
            let base_path = Arc::clone(&base_path);
            tasks.spawn_blocking(move || {
                Properties::new(repo.to_str().expect("Invalid UTF-8 path"), &base_path)
            });
        }

        let mut statuses: Vec<Properties> = Vec::new();

        while let Some(result) = tasks.join_next().await {
            match result {
                Ok(Some(status)) => statuses.push(status),
                Ok(None) => {}
                Err(e) => eprintln!("Task failed: {e}"),
            }
        }

        statuses.sort_by(|a, b| a.name.cmp(&b.name));
        Self {
            props: statuses,
            lens: PropertyLengths::default(),
        }
    }

    pub fn compute_lengths(&mut self) {
        let digit_len = |n: usize| if n == 0 { 1 } else { (n as f64).log10().floor() as usize + 1 };

        self.props.iter().for_each(|s| {
            self.lens.path = max(self.lens.path, s.relative_path.len());
            self.lens.name = max(self.lens.name, s.name.len());
            self.lens.branch = max(self.lens.branch, s.branch.len());
            self.lens.branch_count = max(self.lens.branch_count, digit_len(s.branch_count));
            self.lens.commit_count = max(self.lens.commit_count, digit_len(s.commit_count));
            self.lens.author_name = max(self.lens.author_name, s.author_name.len());
            self.lens.author_email = max(self.lens.author_email, s.author_email.len());
            self.lens.relative_time = max(self.lens.relative_time, s.relative_time.len());
            self.lens.absolute_time = max(self.lens.absolute_time, s.absolute_time.len());
            self.lens.bare = max(self.lens.bare, s.bare.len());
            self.lens.cs_author_count =
                max(self.lens.cs_author_count, digit_len(s.contribution_summary.author_count));
            self.lens.cs_top_commit_count = max(
                self.lens.cs_top_commit_count,
                digit_len(s.contribution_summary.top_commit_count),
            );
            self.lens.cs_top_author_name =
                max(self.lens.cs_top_author_name, s.contribution_summary.top_author_name.len());
            self.lens.cs_top_author_email =
                max(self.lens.cs_top_author_email, s.contribution_summary.top_author_email.len());
        });
    }
}
