use crate::gitter::CLAP_STYLE;

pub fn print_filter_help() {
    let header = CLAP_STYLE.get_header();
    let usage = CLAP_STYLE.get_usage();
    let literal = CLAP_STYLE.get_literal();

    let padding = 40; // Adjust padding as needed for alignment

    println!("{header}Filter Syntax Help{header:#}");

    println!("\n{usage}Description:{usage:#}");
    println!("  Filter expressions allow you to select repositories based on various criteria.");
    println!("  You can combine multiple filter clauses using boolean logic.");

    println!("\n{usage}General Syntax:{usage:#}");
    println!(
        "  {: <padding$} - Combines expressions with logical AND.",
        format!("{literal}<filter_clause> && <filter_clause>{literal:#}")
    );
    println!(
        "  {: <padding$} - Combines expressions with logical OR.",
        format!("{literal}<filter_clause> || <filter_clause>{literal:#}")
    );
    println!(
        "  {: <padding$} - Negates a filter clause or an expression.",
        format!("{literal}! <filter_clause>{literal:#}")
    );
    println!(
        "  {: <padding$} - Groups expressions.",
        format!("{literal}(<expression>){literal:#}")
    );
    println!();
    println!("  Example: {literal}name:project+ && (branch:dev || branch:feature+){literal:#}");

    println!("\n{usage}Filter Clause Format:{usage:#}");
    println!(
        "  A filter clause follows the format: {literal}[!] <prefix>:<value_pattern>{literal:#}"
    );

    println!("\n{header}Prefixes:{header:#}");
    println!("  - {literal}path{literal:#}: Filters by the relative path of the repository.");
    println!("  - {literal}name{literal:#}: Filters by the name of the repository.");
    println!("  - {literal}branch{literal:#}: Filters by the current branch name.");

    println!("\n{header}Value Patterns:{header:#}");
    println!("  - {literal}value{literal:#}: Exact match.");
    println!(
        "  - {literal}value+{literal:#}: Starts with {literal}value{literal:#} (prefix match)."
    );
    println!("  - {literal}+value{literal:#}: Ends with {literal}value{literal:#} (suffix match).");
    println!(
        "  - {literal}+value+{literal:#}: Contains {literal}value{literal:#} (substring match)."
    );

    println!("\n{header}Examples:{header:#}");
    println!("  - {literal}name:my-repo{literal:#}");
    println!("    Matches repositories with the exact name \"my-repo\".");
    println!();
    println!("  - {literal}path:src/projects+{literal:#}");
    println!("    Matches repositories whose relative path starts with \"src/projects\".");
    println!();
    println!("  - {literal}branch:+main{literal:#}");
    println!("    Matches repositories whose current branch ends with \"main\".");
    println!();
    println!("  - {literal}name:+feature+{literal:#}");
    println!("    Matches repositories whose name contains \"feature\".");
    println!();
    println!("  - {literal}name:repo1 || name:repo2{literal:#}");
    println!("    Matches repositories named \"repo1\" OR \"repo2\".");
    println!();
    println!("  - {literal}name:project+ && branch:main{literal:#}");
    println!(
        "    Matches repositories whose name starts with \"project\" AND are on the \"main\" branch."
    );
    println!();
    println!("  - {literal}!branch:main{literal:#}");
    println!("    Matches repositories that are NOT on the \"main\" branch.");
    println!();
    println!("  - {literal}!(name:test+ || name:temp+){literal:#}");
    println!("    Matches repositories whose name does NOT start with \"test\" OR \"temp\".");
    println!();
    println!("  - {literal}path:backend+ && (branch:dev || branch:feature+){literal:#}");
    println!(
        "    Matches backend repositories that are on the \"dev\" branch OR a branch starting with \"feature\"."
    );
}
