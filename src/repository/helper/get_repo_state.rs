use git2::{Repository, RepositoryState};

// "" for a clean state, otherwise merge, rebase, cherry-pick, revert, bisect or mailbox
pub fn get_repo_state(repository: &Repository) -> String {
    match repository.state() {
        RepositoryState::Clean => "",
        RepositoryState::Merge => "merge",
        RepositoryState::Revert | RepositoryState::RevertSequence => "revert",
        RepositoryState::CherryPick | RepositoryState::CherryPickSequence => "cherry-pick",
        RepositoryState::Bisect => "bisect",
        RepositoryState::Rebase
        | RepositoryState::RebaseInteractive
        | RepositoryState::RebaseMerge => "rebase",
        RepositoryState::ApplyMailbox | RepositoryState::ApplyMailboxOrRebase => "mailbox",
    }
    .to_string()
}

pub fn get_shallow(repository: &Repository) -> String {
    if repository.is_shallow() { "shallow".to_string() } else { String::new() }
}
