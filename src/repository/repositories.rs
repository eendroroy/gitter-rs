use crate::cli::gitter::RepoArgs;
use crate::repository::filter_repositories::Filter;
use crate::repository::needed::Needed;
use crate::placeholder::processor::{evaluate_placeholders, replace_placeholders};
use crate::repository::helper::{
    get_absolute_path, get_absolute_time, get_bare, get_branch_count, get_commit_count,
    get_current_branch, get_current_commit_info, get_dirty, get_relative_path, get_relative_time,
    get_commit_summary, get_remote, get_remote_count, get_repo_name, get_repo_size, get_repo_state,
    get_shallow, get_stash_count, get_tag_count, get_top_language, get_tracking, get_user,
    get_worktree_count,
};
use std::cmp::max;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

const MAX_CONCURRENT_TASKS: usize = 20;

#[derive(Debug, Default, Clone)]
pub struct Properties {
    pub repo_path: String, // to use as repo working directory, not a placeholder
    pub absolute_path: String,
    pub relative_path: String,
    pub nesting: usize,
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
    pub relative_time_combined: String,
    pub absolute_time: String,
    pub dirty: String,
    pub is_dirty: bool,
    pub bare: String,
    pub is_bare: bool,
    pub top_lang: String,
    pub upstream: String,
    pub ahead: String,
    pub behind: String,
    pub repo_state: String,
    pub shallow: String,
    pub user_name: String,
    pub user_email: String,
    pub commit_summary: String,
    pub remote_count: usize,
    pub tag_count: usize,
    pub worktree_count: usize,
    pub stash_count: usize,
}

impl Properties {
    /// Opens the repository and computes the always-cheap fields plus those in `needed`.
    pub fn new(path: &Path, base_path: &Path, needed: Needed) -> Option<Self> {
        let mut repository = git2::Repository::open(path).ok()?;

        let (relative_path, nesting) = get_relative_path(path, base_path);
        let (bare, is_bare) = get_bare(&repository);

        let mut props = Self {
            repo_path: path.display().to_string(),
            absolute_path: get_absolute_path(path),
            relative_path,
            nesting,
            name: get_repo_name(path),
            bare,
            is_bare,
            ..Default::default()
        };
        props.fill(&mut repository, needed);
        Some(props)
    }

    /// Computes additional fields on an already built `Properties`.
    pub fn extend(&mut self, needed: Needed) {
        if let Ok(mut repository) = git2::Repository::open(&self.repo_path) {
            self.fill(&mut repository, needed);
        }
    }

    fn fill(&mut self, repository: &mut git2::Repository, needed: Needed) {
        if needed.has(Needed::REMOTE) {
            (self.remote_name, self.remote_fetch, self.remote_push) = get_remote(repository);
        }
        if needed.has(Needed::BRANCH) {
            self.branch = get_current_branch(repository);
        }
        if needed.has(Needed::BRANCH_COUNT) {
            self.branch_count = get_branch_count(repository);
        }
        if needed.has(Needed::COMMIT_COUNT) {
            self.commit_count = get_commit_count(repository);
        }
        if needed.has(Needed::DIRTY) {
            (self.dirty, self.is_dirty) = get_dirty(repository);
        }
        if needed.has(Needed::SIZE) {
            self.repo_size = get_repo_size(repository);
        }
        if needed.has(Needed::LANGUAGE) {
            self.top_lang = get_top_language(repository);
        }
        if needed.has(Needed::TRACKING) {
            (self.upstream, self.ahead, self.behind) = get_tracking(repository);
        }
        if needed.has(Needed::STATE) {
            self.repo_state = get_repo_state(repository);
            self.shallow = get_shallow(repository);
        }
        if needed.has(Needed::USER) {
            (self.user_name, self.user_email) = get_user(repository);
        }
        if needed.has(Needed::COUNTS) {
            self.remote_count = get_remote_count(repository);
            self.tag_count = get_tag_count(repository);
            self.worktree_count = get_worktree_count(repository);
            self.stash_count = get_stash_count(repository);
        }
        if needed.has(Needed::COMMIT) || needed.has(Needed::SUMMARY) {
            let head_commit = repository.head().ok().and_then(|h| h.peel_to_commit().ok());
            let head_commit = head_commit.as_ref();
            if needed.has(Needed::COMMIT) {
                (self.commit_hash, self.author_name, self.author_email) =
                    get_current_commit_info(head_commit);
                (self.relative_time, self.relative_time_combined) = get_relative_time(head_commit);
                self.absolute_time = get_absolute_time(head_commit);
            }
            if needed.has(Needed::SUMMARY) {
                self.commit_summary = get_commit_summary(head_commit);
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PropertyLengths {
    pub absolute_path: usize,
    pub relative_path: usize,
    pub nesting: usize,
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
    pub relative_time_combined: usize,
    pub absolute_time: usize,
    pub dirty: usize,
    pub bare: usize,
    pub top_lang: usize,
    pub upstream: usize,
    pub ahead: usize,
    pub behind: usize,
    pub repo_state: usize,
    pub shallow: usize,
    pub user_name: usize,
    pub user_email: usize,
    pub commit_summary: usize,
    pub remote_count: usize,
    pub tag_count: usize,
    pub worktree_count: usize,
    pub stash_count: usize,
}

#[derive(Debug, Clone)]
pub struct Repositories {
    pub props: Vec<Properties>,
    pub lens: PropertyLengths,
}

/// Runs `work` over `items` on the blocking pool with bounded concurrency; result order is unspecified.
async fn run_bounded<T, F>(items: Vec<T>, work: F) -> Vec<Properties>
where
    T: Send + 'static,
    F: Fn(T) -> Option<Properties> + Send + Sync + 'static,
{
    let work = Arc::new(work);
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_TASKS));
    let mut tasks = JoinSet::new();
    let count = items.len();

    for item in items {
        let work = Arc::clone(&work);
        let sem = Arc::clone(&semaphore);

        tasks.spawn(async move {
            let _permit = sem.acquire_owned().await.ok()?;
            tokio::task::spawn_blocking(move || work(item)).await.ok().flatten()
        });
    }

    let mut results = Vec::with_capacity(count);
    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(Some(props)) => results.push(props),
            Ok(None) => {}
            Err(e) => eprintln!("Task panicked or was canceled: {e}"),
        }
    }
    results
}

impl Repositories {
    /// Cheap fields needed by the filter are computed first; the rest (`display`) only for repositories that pass.
    pub async fn new(
        repositories: Vec<PathBuf>,
        path: &Path,
        filter: Option<&Filter>,
        display: Needed,
    ) -> Self {
        let base_path = Arc::new(path.to_owned());
        let first = filter.map_or(display, Filter::needed);

        let mut props =
            run_bounded(repositories, move |repo| Properties::new(&repo, &base_path, first)).await;

        if let Some(filter) = filter {
            props.retain(|p| filter.matches(p));
        }

        let remaining = display.without(first);
        if !remaining.is_empty() {
            props = run_bounded(props, move |mut p| {
                p.extend(remaining);
                Some(p)
            })
            .await;
        }

        Self {
            props,
            lens: PropertyLengths::default(),
        }
    }

    pub fn sort(&mut self, repo: &RepoArgs) {
        self.props.sort_by_cached_key(|p| {
            replace_placeholders(&repo.sort, &evaluate_placeholders(&repo.sort, p))
        });

        if repo.reverse {
            self.props.reverse();
        }
    }

    pub fn compute_lengths(&mut self) {
        let digit_len = |n: usize| if n == 0 { 1 } else { (n as f64).log10().floor() as usize + 1 };

        self.props.iter().for_each(|s| {
            self.lens.absolute_path = max(self.lens.absolute_path, s.absolute_path.len());
            self.lens.relative_path = max(self.lens.relative_path, s.relative_path.len());
            self.lens.nesting = max(self.lens.nesting, digit_len(s.nesting));
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
            self.lens.relative_time_combined =
                max(self.lens.relative_time_combined, s.relative_time_combined.len());
            self.lens.absolute_time = max(self.lens.absolute_time, s.absolute_time.len());
            self.lens.dirty = max(self.lens.dirty, s.dirty.len());
            self.lens.bare = max(self.lens.bare, s.bare.len());
            self.lens.top_lang = max(self.lens.top_lang, s.top_lang.len());
            self.lens.upstream = max(self.lens.upstream, s.upstream.len());
            self.lens.ahead = max(self.lens.ahead, s.ahead.len());
            self.lens.behind = max(self.lens.behind, s.behind.len());
            self.lens.repo_state = max(self.lens.repo_state, s.repo_state.len());
            self.lens.shallow = max(self.lens.shallow, s.shallow.len());
            self.lens.user_name = max(self.lens.user_name, s.user_name.len());
            self.lens.user_email = max(self.lens.user_email, s.user_email.len());
            self.lens.commit_summary = max(self.lens.commit_summary, s.commit_summary.len());
            self.lens.remote_count = max(self.lens.remote_count, digit_len(s.remote_count));
            self.lens.tag_count = max(self.lens.tag_count, digit_len(s.tag_count));
            self.lens.worktree_count = max(self.lens.worktree_count, digit_len(s.worktree_count));
            self.lens.stash_count = max(self.lens.stash_count, digit_len(s.stash_count));
        });
    }
}
