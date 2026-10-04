use crate::cli::gitter::RepoArgs;
use crate::cli::processor::helper::find_repos;
use crate::repository::helper::DETACHED;
use crate::repository::repositories::Properties;
use crate::style::Palette;
use colored::Colorize;
use git2::Repository;
use std::collections::HashSet;
use tokei::{Config, Languages};

#[derive(Debug, Default)]
struct CodeStats {
    lines_of_code: usize,
    languages: Vec<(String, usize)>,
    tags: usize,
    contributors: usize,
}

fn collect_stats(path: &str) -> CodeStats {
    let Ok(repository) = Repository::open(path) else {
        return CodeStats::default();
    };

    let root = repository
        .workdir()
        .map(|dir| dir.to_path_buf())
        .or_else(|| repository.path().parent().map(|dir| dir.to_path_buf()));
    let mut languages = Languages::new();
    if let Some(root) = root {
        languages.get_statistics(&[root], &[], &Config::from_config_files());
    }

    let mut by_language: Vec<(String, usize)> = languages
        .iter()
        .filter(|(_, stats)| stats.code > 0)
        .map(|(language, stats)| (language.to_string(), stats.code))
        .collect();
    by_language.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let tags = repository.tag_names(None).map(|names| names.len()).unwrap_or(0);

    let mut authors = HashSet::new();
    if let Ok(mut walk) = repository.revwalk()
        && walk.push_head().is_ok()
    {
        for oid in walk.flatten() {
            if let Ok(commit) = repository.find_commit(oid) {
                authors.insert(commit.author().email().unwrap_or_default().to_string());
            }
        }
    }

    CodeStats {
        lines_of_code: by_language.iter().map(|(_, lines)| lines).sum(),
        languages: by_language,
        tags,
        contributors: authors.len(),
    }
}

pub async fn metadata(repo: &RepoArgs) {
    let repos = find_repos(repo).await;
    let styles = Palette::default();

    let handles: Vec<_> = repos
        .props
        .iter()
        .map(|properties| {
            let path = properties.repo_path.clone();
            tokio::task::spawn_blocking(move || collect_stats(&path))
        })
        .collect();

    for (index, (properties, handle)) in repos.props.iter().zip(handles).enumerate() {
        let stats = handle.await.unwrap_or_default();
        if index > 0 {
            println!();
        }
        print_repository(properties, &stats, &styles);
    }
}

fn field(label: &str, value: impl std::fmt::Display) {
    println!("  {:<14} {}", format!("{label}:").bold(), value);
}

fn or_unknown(value: &str) -> &str {
    if value.is_empty() { "unknown" } else { value }
}

fn print_repository(properties: &Properties, stats: &CodeStats, styles: &Palette) {
    let location = if properties.relative_path.is_empty() {
        String::new()
    } else {
        format!(" {}", styles.path.apply(&properties.relative_path))
    };
    println!("{}{}", styles.name.apply(&properties.name), location);

    field("Remote", styles.remote_fetch.apply(or_unknown(&properties.remote_fetch)));
    let branch_style = if properties.branch == DETACHED {
        &styles.detached
    } else {
        &styles.branch
    };
    field("Branch", branch_style.apply(or_unknown(&properties.branch)));
    field(
        "State",
        match (properties.is_bare, properties.is_dirty) {
            (true, _) => "bare".bright_red().bold(),
            (false, true) => "dirty".red().bold(),
            (false, false) => "clean".green().bold(),
        },
    );
    field(
        "Last commit",
        format!(
            "{} {}",
            styles.absolute_time.apply(or_unknown(&properties.absolute_time)),
            format!("({})", or_unknown(properties.relative_time.trim())).dimmed()
        ),
    );
    field(
        "Last author",
        format!(
            "{} {}",
            styles.author_name.apply(or_unknown(&properties.author_name)),
            styles.author_email.apply(&properties.author_email)
        ),
    );
    field("Commits", properties.commit_count.to_string().bold());
    field("Branches", properties.branch_count.to_string().bold());
    field("Tags", stats.tags.to_string().bold());
    field("Contributors", stats.contributors.to_string().bold());
    field("Size", styles.repo_size.apply(or_unknown(&properties.repo_size)));
    field("Lines of code", stats.lines_of_code.to_string().bold());

    let languages = if stats.languages.is_empty() {
        "none".to_string()
    } else {
        stats
            .languages
            .iter()
            .map(|(language, lines)| format!("{} ({lines})", styles.top_lang.apply(language)))
            .collect::<Vec<_>>()
            .join(", ")
    };
    field("Languages", languages);
}
