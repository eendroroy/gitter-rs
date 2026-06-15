use crate::gitter::CLAP_STYLE;

pub fn print_gitterignore_help() {
    let header = CLAP_STYLE.get_header();
    let usage = CLAP_STYLE.get_usage();
    let literal = CLAP_STYLE.get_literal();

    let padding = 40;

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
        "  {: <padding$} - Ignores repositories with the exact name.",
        format!("{literal}repo_name{literal:#}")
    );
    println!(
        "  {: <padding$} - Ignores repositories with the exact relative path.",
        format!("{literal}relative/path/to/repo{literal:#}")
    );
    println!(
        "  {: <padding$} - Ignores if any path component is 'dir_name'.",
        format!("{literal}*/dir_name{literal:#}")
    );
    println!(
        "  {: <padding$} - Ignores if the top-level directory is 'dir_name'.",
        format!("{literal}dir_name/*{literal:#}")
    );

    println!("\n{usage}Examples:{usage:#}");
    println!("  # Ignore a specific repository by exact name");
    println!("  my_secret_repo");
    println!();
    println!("  # Ignore a repository by its exact relative path");
    println!("  src/projects/old_stuff");
    println!();
    println!("  # Ignore repositories if any path component is 'build'");
    println!("  */build");
    println!();
    println!("  # Ignore repositories if their top-level directory is 'vendor'");
    println!("  vendor/*");
    println!();
}
