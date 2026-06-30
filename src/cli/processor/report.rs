use crate::cli::gitter::{RepoArgs, ReportArgs};
use crate::cli::processor::helper::find_repos;
use crate::repository::print_info::print_info_line;
use chrono::{TimeZone, Utc};
use git2::{Repository, Sort};
use std::collections::BTreeMap;

pub async fn report(repo: &RepoArgs, args: &ReportArgs) {
    if args.commit_graph {
        commit_graph(repo, args).await
    }
}

async fn commit_graph(repo: &RepoArgs, args: &ReportArgs) {
    let repos = find_repos(repo).await;

    fn get_repo_commit(path: String) -> BTreeMap<String, usize> {
        let mut commit_dataset: BTreeMap<String, usize> = BTreeMap::new();

        let repo = match Repository::open(&path) {
            Ok(r) => r,
            Err(_) => return commit_dataset,
        };

        let mut revwalk = match repo.revwalk() {
            Ok(rw) => rw,
            Err(_) => return commit_dataset,
        };

        if revwalk.push_head().is_err() {
            return commit_dataset;
        }
        let _ = revwalk.set_sorting(Sort::TIME);

        for id in revwalk {
            if let Ok(commit_id) = id
                && let Ok(commit) = repo.find_commit(commit_id)
            {
                let seconds = commit.time().seconds();
                let datetime = Utc.timestamp_opt(seconds, 0).unwrap();
                let date_string = datetime.format("%Y-%m-%d").to_string();

                *commit_dataset.entry(date_string).or_insert(0) += 1;
            }
        }
        commit_dataset
    }

    let mut tasks = Vec::new();

    for status in repos.props.iter() {
        let path = status.repo_path.clone();
        let status_clone = status.clone();

        let task = tokio::spawn(async move {
            let dataset = tokio::task::spawn_blocking(move || get_repo_commit(path))
                .await
                .unwrap_or_default();
            (status_clone, dataset)
        });

        tasks.push(task);
    }

    let mut processed_repos = Vec::new();
    for task in tasks {
        if let Ok(result) = task.await {
            processed_repos.push(result);
        }
    }

    for (status, commit_dataset) in processed_repos {
        print_info_line(
            &repo.info_template,
            &status,
            Some(repos.lens),
            &repo.align,
            &args.show_info,
        );

        if commit_dataset.is_empty() {
            println!("No commits found or repository inaccessible.");
            println!("—");
            continue;
        }

        let keys: Vec<&String> = commit_dataset.keys().collect();
        let all_counts: Vec<usize> = commit_dataset.values().cloned().collect();

        let side_padding = 12;
        let total_date_overhead = side_padding * 2;

        let graph_width = 10;

        let slice_start = if all_counts.len() > graph_width {
            all_counts.len() - graph_width
        } else {
            0
        };

        let display_counts = &all_counts[slice_start..];
        let start_date = keys[slice_start];
        let end_date = keys[keys.len() - 1];

        let max_commits = display_counts.iter().cloned().max().unwrap_or(1);
        let max_height = 1;
        let symbols = [" ", " ", "▂", "▃", "▄", "▅", "▆", "▇", "█"];

        print!(" {} ", start_date);

        for level in (0..max_height).rev() {
            for &count in display_counts {
                let total_eighths = (count * max_height * 8) / max_commits;
                let level_start_eighths = level * 8;

                if total_eighths >= level_start_eighths + 8 {
                    print!("{}", symbols[8]);
                } else if total_eighths > level_start_eighths {
                    let index = total_eighths - level_start_eighths;
                    print!("{}", symbols[index]);
                } else {
                    print!("{}", symbols[0]);
                }
            }
        }

        println!(" {} ", end_date);

        let total_line_length = display_counts.len() + total_date_overhead;
        println!("{}", "—".repeat(total_line_length));
    }
}
