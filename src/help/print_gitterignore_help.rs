use crate::gitter::CLAP_STYLE;

pub fn print_gitterignore_help() {
    let header = CLAP_STYLE.get_header();
    let usage = CLAP_STYLE.get_usage();
    let literal = CLAP_STYLE.get_literal();

    let padding = 25; // Adjusted padding for better alignment

    println!("{header}Gitterignore File Format{header:#}");

    println!("\n{usage}Description:{usage:#}");
    println!(
        "  The {literal}.gitterignore{literal:#} file allows you to specify repositories that should be ignored by gitter."
    );
    println!(
        "  It supports various patterns based on repository name, path, and parent directory."
    );
    println!(
        "  Rules are applied hierarchically: a {literal}.gitterignore{literal:#} file in a subdirectory will apply"
    );
    println!(
        "  to that directory and its children, potentially overriding rules from parent directories."
    );

    println!("\n{usage}Rule Format:{usage:#}");
    println!("  Each line in the {literal}.gitterignore{literal:#} file represents a single rule.");
    println!("  Empty lines and lines starting with {literal}#{literal:#} are ignored (comments).");
    println!("  Rules are defined using a {literal}key:pattern{literal:#} format.");

    println!("\n{header}Available Rule Types:{header:#}");

    println!("\n{literal}name:{literal:#}");
    println!("  Matches against the repository's name (the directory name of the repository).");
    println!(
        "  {: <padding$} - Ignores repositories with the exact name \"exact_name\".",
        format!("{literal}name:exact_name{literal:#}")
    );
    println!(
        "  {: <padding$} - Ignores repositories whose name starts with \"prefix\".",
        format!("{literal}name:prefix*{literal:#}")
    );
    println!(
        "  {: <padding$} - Ignores repositories whose name ends with \"suffix\".",
        format!("{literal}name:*suffix{literal:#}")
    );
    println!(
        "  {: <padding$} - Ignores repositories whose name contains \"contains\".",
        format!("{literal}name:*contains*{literal:#}")
    );

    println!("\n{literal}path:{literal:#}");
    println!("  Matches against the repository's absolute path.");
    println!(
        "  {: <padding$} - Ignores repositories with the exact absolute path \"exact_path\".",
        format!("{literal}path:/home/user/repo{literal:#}")
    );
    println!(
        "  {: <padding$} - Ignores repositories whose path starts with \"prefix\".",
        format!("{literal}path:/home/user/prefix*{literal:#}")
    );
    println!(
        "  {: <padding$} - Ignores repositories whose path ends with \"suffix\".",
        format!("{literal}path:*suffix{literal:#}")
    );
    println!(
        "  {: <padding$} - Ignores repositories whose path contains \"contains\".",
        format!("{literal}path:*contains*{literal:#}")
    );

    println!("\n{literal}child:{literal:#}");
    println!("  Matches against the immediate parent directory name of the repository.");
    println!(
        "  {: <padding$} - Ignores repositories whose immediate parent directory is \"parent_dir\".",
        format!("{literal}child:parent_dir{literal:#}")
    );

    println!("\n{usage}Example .gitterignore:{usage:#}");
    println!("  # Ignore a specific repository by name");
    println!("  name:my_secret_repo");
    println!();
    println!("  # Ignore all repositories whose name starts with 'temp'");
    println!("  name:temp*");
    println!();
    println!("  # Ignore repositories in a specific absolute path");
    println!("  path:/home/user/projects/old_stuff");
    println!();
    println!("  # Ignore all repositories directly under a 'node_modules' directory");
    println!("  child:node_modules");
    println!();
}
