use crate::gitter::Gitter;
use crate::placeholder::processor::{evaluate_placeholders, replace_placeholders};
use crate::repository::helper::{
    get_absolute_path, get_absolute_time, get_bare, get_branch_count, get_commit_count,
    get_contributor_summary, get_current_branch, get_current_commit_info, get_dirty,
    get_relative_path, get_relative_time, get_repo_name, get_repo_size,
};
use git2::Repository;
use std::cmp::max;
use std::path::{Path, PathBuf};
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
    pub repo_size: String,
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
    pub cs: ContributionSummary,
}

impl Properties {
    pub fn new(path: &Path, base_path: &Path) -> Option<Self> {
        let repository = Repository::open(path).ok()?;
        let absolute_path = get_absolute_path(path);
        let relative_path = get_relative_path(path, base_path);
        let repo_size = get_repo_size(&repository);
        let name = get_repo_name(path);
        let branch = get_current_branch(&repository);
        let branch_count = get_branch_count(&repository);
        let (commit_hash, author_name, author_email) = get_current_commit_info(&repository);
        let commit_count = get_commit_count(&repository);
        let relative_time = get_relative_time(&repository);
        let absolute_time = get_absolute_time(&repository);
        let (dirty, is_dirty) = get_dirty(&repository);
        let (bare, is_bare) = get_bare(&repository);
        let cs = get_contributor_summary(&repository);

        Some(Self {
            absolute_path,
            relative_path,
            repo_size,
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
            cs,
        })
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PropertyLengths {
    pub absolute_path: usize,
    pub relative_path: usize,
    pub repo_size: usize,
    pub name: usize,
    pub branch: usize,
    pub branch_count: usize,
    pub commit_hash: usize,
    pub commit_count: usize,
    pub author_name: usize,
    pub author_email: usize,
    pub relative_time: usize,
    pub absolute_time: usize,
    pub bare: usize,
    pub cs_author_count: usize,
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
    pub async fn new(repositories: Vec<PathBuf>, path: &Path) -> Self {
        let base_path = Arc::new(path.to_owned());

        let mut tasks = JoinSet::new();

        for repo in repositories {
            let base_path = Arc::clone(&base_path);
            tasks.spawn_blocking(move || Properties::new(&repo, &base_path));
        }

        let mut statuses: Vec<Properties> = Vec::new();

        while let Some(result) = tasks.join_next().await {
            match result {
                Ok(Some(status)) => statuses.push(status),
                Ok(None) => {}
                Err(e) => eprintln!("Task failed: {e}"),
            }
        }

        Self {
            props: statuses,
            lens: PropertyLengths::default(),
        }
    }

    pub fn sort(&mut self, cli: &Gitter) {
        self.props.sort_by(|a, b| {
            replace_placeholders(&cli.sort, &evaluate_placeholders(&cli.sort, a))
                .cmp(&replace_placeholders(&cli.sort, &evaluate_placeholders(&cli.sort, b)))
        });

        if cli.reverse {
            self.props.reverse();
        }
    }

    pub fn compute_lengths(&mut self) {
        let digit_len = |n: usize| if n == 0 { 1 } else { (n as f64).log10().floor() as usize + 1 };

        self.props.iter().for_each(|s| {
            self.lens.absolute_path = max(self.lens.absolute_path, s.absolute_path.len());
            self.lens.relative_path = max(self.lens.relative_path, s.relative_path.len());
            self.lens.repo_size = max(self.lens.repo_size, s.repo_size.len());
            self.lens.name = max(self.lens.name, s.name.len());
            self.lens.branch = max(self.lens.branch, s.branch.len());
            self.lens.branch_count = max(self.lens.branch_count, digit_len(s.branch_count));
            self.lens.commit_hash = max(self.lens.commit_hash, s.commit_hash.len());
            self.lens.commit_count = max(self.lens.commit_count, digit_len(s.commit_count));
            self.lens.author_name = max(self.lens.author_name, s.author_name.len());
            self.lens.author_email = max(self.lens.author_email, s.author_email.len());
            self.lens.relative_time = max(self.lens.relative_time, s.relative_time.len());
            self.lens.absolute_time = max(self.lens.absolute_time, s.absolute_time.len());
            self.lens.bare = max(self.lens.bare, s.bare.len());
            self.lens.cs_author_count =
                max(self.lens.cs_author_count, digit_len(s.cs.author_count));
            self.lens.cs_top_commit_count =
                max(self.lens.cs_top_commit_count, digit_len(s.cs.top_commit_count));
            self.lens.cs_top_author_name =
                max(self.lens.cs_top_author_name, s.cs.top_author_name.len());
            self.lens.cs_top_author_email =
                max(self.lens.cs_top_author_email, s.cs.top_author_email.len());
        });
    }
}
