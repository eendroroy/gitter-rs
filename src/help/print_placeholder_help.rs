use crate::gitter::CLAP_STYLE;

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
    println!("  Example: gitter bash -- echo \"Current branch name is: {{_branch:n_}}\"");

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
        format!("{literal}{{_branch:n_}}{literal:#}")
    );
    println!(
        "  {: <padding$} Total number of branches",
        format!("{literal}{{_branch:c_}}{literal:#}")
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
}
