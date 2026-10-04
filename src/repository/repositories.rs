use crate::cli::gitter::RepoArgs;
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
    pub fn new(path: &Path, base_path: &Path) -> Option<Self> {
        let mut repository = git2::Repository::open(path).ok()?;
        let stash_count = get_stash_count(&mut repository);
        let head_commit = repository.head().ok().and_then(|h| h.peel_to_commit().ok());
        let head_commit = head_commit.as_ref();

        let absolute_path = get_absolute_path(path);
        let (relative_path, nesting) = get_relative_path(path, base_path);
        let name = get_repo_name(path);
        let repo_size = get_repo_size(&repository);
        let (remote_name, remote_fetch, remote_push) = get_remote(&repository);

        let branch = get_current_branch(&repository);
        let branch_count = get_branch_count(&repository);

        let (commit_hash, author_name, author_email) = get_current_commit_info(head_commit);
        let commit_count = get_commit_count(&repository);
        let (relative_time, relative_time_combined) = get_relative_time(head_commit);
        let absolute_time = get_absolute_time(head_commit);

        let (dirty, is_dirty) = get_dirty(&repository);
        let (bare, is_bare) = get_bare(&repository);

        let top_lang = get_top_language(&repository);

        let (upstream, ahead, behind) = get_tracking(&repository);
        let repo_state = get_repo_state(&repository);
        let shallow = get_shallow(&repository);
        let (user_name, user_email) = get_user(&repository);
        let commit_summary = get_commit_summary(head_commit);
        let remote_count = get_remote_count(&repository);
        let tag_count = get_tag_count(&repository);
        let worktree_count = get_worktree_count(&repository);

        Some(Self {
            repo_path: path.display().to_string(),
            absolute_path,
            relative_path,
            nesting,
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
            relative_time_combined,
            absolute_time,
            dirty,
            is_dirty,
            bare,
            is_bare,
            top_lang,
            upstream,
            ahead,
            behind,
            repo_state,
            shallow,
            user_name,
            user_email,
            commit_summary,
            remote_count,
            tag_count,
            worktree_count,
            stash_count,
        })
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

impl Repositories {
    pub async fn new(repositories: Vec<PathBuf>, path: &Path) -> Self {
        let base_path = Arc::new(path.to_owned());
        let mut tasks = JoinSet::new();

        let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_TASKS));

        let repo_count = repositories.len();

        for repo in repositories {
            let base_path = Arc::clone(&base_path);
            let sem = Arc::clone(&semaphore);

            tasks.spawn(async move {
                let _permit = sem.acquire_owned().await.ok()?;

                tokio::task::spawn_blocking(move || Properties::new(&repo, &base_path))
                    .await
                    .ok()
                    .flatten()
            });
        }

        let mut statuses: Vec<Properties> = Vec::with_capacity(repo_count);

        while let Some(result) = tasks.join_next().await {
            match result {
                Ok(Some(status)) => statuses.push(status),
                Ok(None) => {}
                Err(e) => eprintln!("Task panicked or was canceled: {e}"),
            }
        }

        Self {
            props: statuses,
            lens: PropertyLengths::default(),
        }
    }

    pub fn sort(&mut self, repo: &RepoArgs) {
        self.props.sort_by(|a, b| {
            replace_placeholders(&repo.sort, &evaluate_placeholders(&repo.sort, a))
                .cmp(&replace_placeholders(&repo.sort, &evaluate_placeholders(&repo.sort, b)))
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
