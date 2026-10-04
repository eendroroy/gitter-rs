use git2::Repository;
use std::sync::LazyLock;
use tokei::{Config, Languages};

static CONFIG: LazyLock<Config> = LazyLock::new(Config::from_config_files);

pub fn get_top_language(repository: &Repository) -> String {
    let config = &*CONFIG;
    let mut languages = Languages::new();

    if let Some(repo_root) = repository.workdir() {
        let excluded: &[&str] = &[];

        languages.get_statistics(&[repo_root], excluded, config);
    } else {
        if let Some(repo_path) = repository.path().parent() {
            languages.get_statistics(&[repo_path], &[], config);
        }
    }

    languages
        .iter()
        .map(|(lang_type, language_stats)| {
            let total_code_lines = language_stats.code;
            (*lang_type, total_code_lines)
        })
        .max_by_key(|&(_, code_lines)| code_lines)
        .filter(|&(_, code_lines)| code_lines > 0)
        .map(|(lang_type, _)| lang_type.to_string())
        .unwrap_or_else(|| "".to_string())
}
