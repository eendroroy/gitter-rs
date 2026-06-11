use crate::gitter::CLAP_STYLE;
use crate::repository::Status;

pub(crate) fn evaluate_placeholders(mut base_string: String, status: &Status) -> String {
    base_string = base_string.replace("{_name_}", status.name.as_str());
    base_string = base_string.replace("{_path:r_}", status.path.as_str());
    base_string = base_string.replace("{_path:a_}", status.absolute_path.as_str());
    base_string = base_string.replace("{_branch_}", status.branch.as_str());
    // base_string = base_string.replace("{_commit:f_}", );
    // base_string = base_string.replace("{_commit:<n>_}", );
    // base_string = base_string.replace("{_commit:c_}", );
    base_string = base_string.replace("{_author:e_}", status.author_email.as_str());
    base_string = base_string.replace("{_author:n_}", status.author_name.as_str());
    base_string = base_string.replace("{_time:r_}", status.relative_time.as_str());
    base_string = base_string.replace("{_time:d_}", status.absolute_time.as_str());

    base_string
}

pub(crate) fn print_placeholder_help() {
    // Extract individual styling definitions from your unified template style
    let header = CLAP_STYLE.get_header();
    let usage = CLAP_STYLE.get_usage();
    let literal = CLAP_STYLE.get_literal();

    let padding = 30;

    // Use '#' to cleanly clear and reset standard console styles at boundary lines
    println!("{header}Gitter Template Placeholders{header:#}");

    println!("\n{usage}Usage:{usage:#}");
    println!(
        "  Pass these tags within string sequences to dynamically extract local repository data."
    );
    println!("  Example: gitter eval -- echo \"Current branch is: {{_branch_}}\"");

    println!("\n{header}Available Placeholders:{header:#}");

    // Maps terms exactly to the look and layout of native clap parameters
    println!(
        "  {: <padding$} {}",
        format!("{literal}{{_name_}}{literal:#}"),
        "The simple name of the repository directory."
    );
    println!(
        "  {: <padding$} {}",
        format!("{literal}{{_path:r_}}{literal:#}"),
        "The relative path from your execution context."
    );
    println!(
        "  {: <padding$} {}",
        format!("{literal}{{_path:a_}}{literal:#}"),
        "The complete absolute file path on the system filesystem."
    );
    println!(
        "  {: <padding$} {}",
        format!("{literal}{{_branch_}}{literal:#}"),
        "The active checked-out Git branch head."
    );
    println!(
        "  {: <padding$} {}",
        format!("{literal}{{_commit:f_}}{literal:#}"),
        "The full 40-character Git commit hash string."
    );
    println!(
        "  {: <padding$} {}",
        format!("{literal}{{_commit:c_}}{literal:#}"),
        "The short 7-character clean commit SHA."
    );
    println!(
        "  {: <padding$} {}",
        format!("{literal}{{_commit:<n>_}}{literal:#}"),
        "A variable length commit SHA slice where 'n' is any integer."
    );
    println!("                           (Example: use {{_commit:12_}} for a 12-character string)");
    println!(
        "  {: <padding$} {}",
        format!("{literal}{{_author:n_}}{literal:#}"),
        "The name signature of the individual behind the latest commit."
    );
    println!(
        "  {: <padding$} {}",
        format!("{literal}{{_author:e_}}{literal:#}"),
        "The email marker boundary of the commit author."
    );
    println!(
        "  {: <padding$} {}",
        format!("{literal}{{_time:r_}}{literal:#}"),
        "The human-readable relative time interval (e.g., '2 hours ago')."
    );
    println!(
        "  {: <padding$} {}",
        format!("{literal}{{_time:d_}}{literal:#}"),
        "The precise absolute date stamp signature format."
    );
    println!();
}
