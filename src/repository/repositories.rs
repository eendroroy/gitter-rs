use crate::gitter::cli::Gitter;
use crate::placeholder::processor::{evaluate_placeholders, replace_placeholders};
use crate::repository::helper::{
    get_absolute_path, get_absolute_time, get_bare, get_branch_count, get_commit_count,
    get_contributor_summary, get_current_branch, get_current_commit_info, get_dirty,
    get_relative_path, get_relative_time, get_remote, get_repo_name, get_repo_size,
    get_top_language,
};
use std::cmp::max;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::task;
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
    pub repo_path: String, // to use as repo working directory, not a placeholder
    pub absolute_path: String,
    pub relative_path: String,
    pub repo_size: String,
    pub remote_name: String,
    pub remote_fetch: String,
    pub remote_push: String,
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
    pub top_lang: String,
}

impl Properties {
    pub async fn new(path: &Path, base_path: &Path) -> Option<Self> {
        let path = path.to_path_buf();
        let base_path = base_path.to_path_buf();

        let path_clone = path.clone();
        let task_basic_info = task::spawn_blocking(move || {
            let repository = git2::Repository::open(&path_clone).ok()?;
            let absolute_path = get_absolute_path(&path_clone);
            let relative_path = get_relative_path(&path_clone, &base_path);
            let name = get_repo_name(&path_clone);
            let repo_size = get_repo_size(&repository);
            let (remote_name, remote_fetch, remote_push) = get_remote(&repository);
            Some((
                absolute_path,
                relative_path,
                name,
                repo_size,
                remote_name,
                remote_fetch,
                remote_push,
            ))
        });

        let path_clone = path.clone();
        let task_branch_info = task::spawn_blocking(move || {
            let repository = git2::Repository::open(&path_clone).ok()?;
            let branch = get_current_branch(&repository);
            let branch_count = get_branch_count(&repository);
            Some((branch, branch_count))
        });

        let path_clone = path.clone();
        let task_commit_info = task::spawn_blocking(move || {
            let repository = git2::Repository::open(&path_clone).ok()?;
            let (commit_hash, author_name, author_email) = get_current_commit_info(&repository);
            let commit_count = get_commit_count(&repository);
            let relative_time = get_relative_time(&repository);
            let absolute_time = get_absolute_time(&repository);
            Some((
                commit_hash,
                author_name,
                author_email,
                commit_count,
                relative_time,
                absolute_time,
            ))
        });

        let path_clone = path.clone();
        let task_status_info = task::spawn_blocking(move || {
            let repository = git2::Repository::open(&path_clone).ok()?;
            let (dirty, is_dirty) = get_dirty(&repository);
            let (bare, is_bare) = get_bare(&repository);
            let cs = get_contributor_summary(&repository);
            Some((dirty, is_dirty, bare, is_bare, cs))
        });

        let path_clone = path.clone();
        let task_language_info = task::spawn_blocking(move || {
            let repository = git2::Repository::open(&path_clone).ok()?;
            let top_lang = get_top_language(&repository);
            Some(top_lang)
        });

        let (basic, branch, commit, status, language) = tokio::join!(
            task_basic_info,
            task_branch_info,
            task_commit_info,
            task_status_info,
            task_language_info
        );

        let (absolute_path, relative_path, name, repo_size, remote_name, remote_fetch, remote_push) =
            basic.ok()??;
        let (branch, branch_count) = branch.ok()??;
        let (commit_hash, author_name, author_email, commit_count, relative_time, absolute_time) =
            commit.ok()??;
        let (dirty, is_dirty, bare, is_bare, cs) = status.ok()??;
        let top_lang = language.ok()??;

        Some(Self {
            repo_path: path.display().to_string(),
            absolute_path,
            relative_path,
            repo_size,
            remote_name,
            remote_fetch,
            remote_push,
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
            top_lang,
        })
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PropertyLengths {
    pub absolute_path: usize,
    pub relative_path: usize,
    pub repo_size: usize,
    pub remote_name: usize,
    pub remote_fetch: usize,
    pub remote_push: usize,
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
    pub top_lang: usize,
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
            tasks.spawn(async move { Properties::new(&repo, &base_path).await });
        }

        let mut statuses: Vec<Properties> = Vec::new();

        while let Some(result) = tasks.join_next().await {
            match result {
                Ok(Some(status)) => statuses.push(status),
                Ok(None) => {}
                Err(e) => eprintln!("Task paniced or was canceled: {e}"),
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
            self.lens.remote_name = max(self.lens.remote_name, s.remote_name.len());
            self.lens.remote_fetch = max(self.lens.remote_fetch, s.remote_fetch.len());
            self.lens.remote_push = max(self.lens.remote_push, s.remote_push.len());
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
            self.lens.top_lang = max(self.lens.top_lang, s.top_lang.len());
        });
    }
}
