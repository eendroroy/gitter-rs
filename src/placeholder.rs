use crate::gitter::CLAP_STYLE;
use crate::repository::Status;
use lazy_static::lazy_static;
use regex::Regex;

pub(crate) fn evaluate_placeholders(mut base_string: String, status: &Status) -> String {
    base_string = base_string.replace("{_name_}", status.name.as_str());
    base_string = base_string.replace("{_path:r_}", status.path.as_str());
    base_string = base_string.replace("{_path:a_}", status.absolute_path.as_str());
    base_string = base_string.replace("{_branch_}", status.branch.as_str());
    base_string = base_string.replace("{_commit:f_}", status.commit_hash.as_str());
    base_string = base_string.replace("{_author:e_}", status.author_email.as_str());
    base_string = base_string.replace("{_author:n_}", status.author_name.as_str());
    base_string = base_string.replace("{_time:r_}", status.relative_time.as_str());
    base_string = base_string.replace("{_time:d_}", status.absolute_time.as_str());

    lazy_static! {
        static ref RE: Regex = Regex::new(r"\{_commit:(\d+)_\}").unwrap();
    }
    base_string = RE
        .replace_all(&base_string, |caps: &regex::Captures| {
            if let Some(num_str) = caps.get(1)
                && let Ok(requested_len) = num_str.as_str().parse::<usize>() {
                    let full_hash = &status.commit_hash;
                    let target_len = std::cmp::min(requested_len, full_hash.len());
                    return full_hash[..target_len].to_string();
                }
            caps.get(0).unwrap().as_str().to_string()
        })
        .into_owned();

    base_string
}

pub(crate) fn print_placeholder_help() {
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
