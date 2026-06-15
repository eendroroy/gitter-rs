use crate::gitter::CLAP_STYLE;

pub fn print_gitterignore_help() {
    let header = CLAP_STYLE.get_header();
    let usage = CLAP_STYLE.get_usage();
    let literal = CLAP_STYLE.get_literal();

    let padding = 30;

    println!("{header}Gitterignore File Format{header:#}");

    println!("\n{usage}Description:{usage:#}");
    println!(
        "  The {literal}.gitterignore{literal:#} file specifies repositories to be ignored by gitter."
    );
    println!(
        "  Each line is a pattern. Empty lines and lines starting with {literal}#{literal:#} are comments."
    );

    println!("\n{header}Patterns:{header:#}");
    println!("  Patterns are interpreted based on their format:");
    println!();
    println!(
        "  {: <padding$} - Ignores repositories with the exact relative path or name.",
        format!("{literal}path/to/repo{literal:#}")
    );
    println!(
        "  {: <padding$} - Ignores repositories whose relative path or name starts with a given prefix.",
        format!("{literal}prefix*{literal:#}")
    );
    println!(
        "  {: <padding$} - Ignores repositories if their immediate sub-directory (relative to the .gitterignore file) is an exact name.",
        format!("{literal}dir_name/*{literal:#}")
    );
    println!(
        "  {: <padding$} - Ignores repositories if their immediate sub-directory (relative to the .gitterignore file) starts with a given prefix.",
        format!("{literal}dir_prefix*/*{literal:#}")
    );
}
