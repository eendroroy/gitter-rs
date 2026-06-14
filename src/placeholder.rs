use crate::gitter::CLAP_STYLE;
use crate::repository::repositories::Properties;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub fn evaluate_placeholders(base_string: String, status: &Properties) -> HashMap<String, String> {
    let mut evaluation = HashMap::new();

    if base_string.contains("{_name_}") {
        evaluation.insert("{_name_}".to_string(), status.name.clone());
    }
    if base_string.contains("{_path:r_}") {
        evaluation.insert("{_path:r_}".to_string(), status.relative_path.clone());
    }
    if base_string.contains("{_path:a_}") {
        evaluation.insert("{_path:a_}".to_string(), status.absolute_path.clone());
    }
    if base_string.contains("{_branch_}") {
        evaluation.insert("{_branch_}".to_string(), status.branch.clone());
    }
    if base_string.contains("{_commit:f_}") {
        evaluation.insert("{_commit:f_}".to_string(), status.commit_hash.clone());
    }
    if base_string.contains("{_commit:c_}") {
        evaluation.insert("{_commit:c_}".to_string(), status.commit_count.to_string());
    }
    if base_string.contains("{_author:e_}") {
        evaluation.insert("{_author:e_}".to_string(), status.author_email.clone());
    }
    if base_string.contains("{_author:n_}") {
        evaluation.insert("{_author:n_}".to_string(), status.author_name.clone());
    }
    if base_string.contains("{_time:r_}") {
        evaluation.insert("{_time:r_}".to_string(), status.relative_time.clone());
    }
    if base_string.contains("{_time:d_}") {
        evaluation.insert("{_time:d_}".to_string(), status.absolute_time.clone());
    }

    lazy_static! {
        static ref RE: Regex = Regex::new(r"\{_commit:(\d+)_\}").unwrap();
    }
    for caps in RE.captures_iter(&base_string) {
        let full_match = caps.get(0).unwrap().as_str();

        if let Some(num_str) = caps.get(1)
            && let Ok(requested_len) = num_str.as_str().parse::<usize>()
        {
            let full_hash = &status.commit_hash;
            let target_len = std::cmp::min(requested_len, full_hash.len());
            let sliced_hash = full_hash[..target_len].to_string();
            evaluation.insert(full_match.to_string(), sliced_hash);
        }
    }

    evaluation
}

pub fn replace_placeholders(
    mut base_string: String,
    evaluation: HashMap<String, String>,
) -> String {
    for (placeholder_tag, evaluated_value) in evaluation {
        base_string = base_string.replace(&placeholder_tag, &evaluated_value);
    }

    base_string
}

pub fn print_placeholder_help() {
    let header = CLAP_STYLE.get_header();
    let usage = CLAP_STYLE.get_usage();
    let literal = CLAP_STYLE.get_literal();

    let padding = 30;

    println!("{header}Gitter Template Placeholders{header:#}");

    println!("\n{usage}Usage:{usage:#}");
    println!(
        "  Pass these tags within string sequences to dynamically extract local repository data."
    );
    println!("  Example: gitter eval -- echo \"Current branch is: {{_branch_}}\"");

    println!("\n{header}Available Placeholders:{header:#}");

    println!(
        "  {: <padding$} The simple name of the repository directory.",
        format!("{literal}{{_name_}}{literal:#}")
    );
    println!(
        "  {: <padding$} The relative path from your execution context.",
        format!("{literal}{{_path:r_}}{literal:#}")
    );
    println!(
        "  {: <padding$} The complete absolute file path on the system filesystem.",
        format!("{literal}{{_path:a_}}{literal:#}")
    );
    println!(
        "  {: <padding$} The active checked-out Git branch head.",
        format!("{literal}{{_branch_}}{literal:#}")
    );
    println!(
        "  {: <padding$} The full 40-character Git commit hash string.",
        format!("{literal}{{_commit:f_}}{literal:#}")
    );
    println!(
        "  {: <padding$} Number of commits in current branch",
        format!("{literal}{{_commit:c_}}{literal:#}")
    );
    println!(
        "  {: <padding$} A variable length commit SHA slice where 'n' is any integer.",
        format!("{literal}{{_commit:<n>_}}{literal:#}")
    );
    println!("                           (Example: use {{_commit:12_}} for a 12-character string)");
    println!(
        "  {: <padding$} The name signature of the individual behind the latest commit.",
        format!("{literal}{{_author:n_}}{literal:#}")
    );
    println!(
        "  {: <padding$} The email marker boundary of the commit author.",
        format!("{literal}{{_author:e_}}{literal:#}")
    );
    println!(
        "  {: <padding$} The human-readable relative time interval (e.g., '2 hours ago').",
        format!("{literal}{{_time:r_}}{literal:#}")
    );
    println!(
        "  {: <padding$} The precise absolute date stamp signature format.",
        format!("{literal}{{_time:d_}}{literal:#}")
    );
    println!();
}
