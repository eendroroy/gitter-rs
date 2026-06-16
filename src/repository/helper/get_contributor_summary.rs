use crate::repository::repositories::ContributionSummary;
use git2::{Repository, Sort};
use std::collections::HashMap;

pub fn get_contributor_summary(repository: &Repository) -> ContributionSummary {
    let mut revwalk = match repository.revwalk() {
        Ok(rw) => rw,
        Err(_) => return ContributionSummary::default(),
    };

    if revwalk.push_head().is_err() || revwalk.set_sorting(Sort::TIME).is_err() {
        return ContributionSummary::default();
    }

    let mut contributor_counts: HashMap<(String, String), usize> = HashMap::new();

    for oid in revwalk.flatten() {
        if let Ok(commit) = repository.find_commit(oid) {
            let author = commit.author();

            if let (Ok(name), Ok(email)) = (author.name(), author.email()) {
                let key = (name.to_string(), email.to_string());
                *contributor_counts.entry(key).or_insert(0) += 1;
            }
        }
    }

    let contribution_count = contributor_counts.values().sum::<usize>();
    let contributor_count = contributor_counts.len();

    match contributor_counts.into_iter().max_by_key(|&(_, count)| count) {
        Some(((top_contributor_name, top_contributor_email), top_contribution_count)) => {
            ContributionSummary {
                author_count: contributor_count,
                commit_count: contribution_count,
                top_commit_count: top_contribution_count,
                top_author_name: top_contributor_name,
                top_author_email: top_contributor_email,
            }
        }
        None => ContributionSummary::default(),
    }
}
