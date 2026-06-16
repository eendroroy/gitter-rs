use crate::gitter::CLAP_STYLE;

macro_rules! placeholder_template {
    () => {
"\
{header}Gitter Template Placeholders{header:#}

{usage}Usage:{usage:#}
  Pass these tags within string sequences to dynamically extract local repository data.
  Example: {literal}gitter bash{literal:#} {placeholder}-- echo \"Current branch name is: {{_branch:n_}}\"{placeholder:#}

{header}Available Placeholders:{header:#}
  {literal}{{_name_}}{literal:#}             The simple name of the repository directory.
  {literal}{{_path:r_}}{literal:#}           The relative path from your execution context.
  {literal}{{_path:a_}}{literal:#}           The complete absolute file path on the system filesystem.
  {literal}{{_branch:n_}}{literal:#}         The active checked-out Git branch head.
  {literal}{{_branch:c_}}{literal:#}         Total number of branches.
  {literal}{{_commit:c_}}{literal:#}         Total number of commits in current branch.
  {literal}{{_hash:f_}}{literal:#}           The full 40-character Git commit hash string.
  {literal}{{_hash:<n>_}}{literal:#}         A variable length commit SHA slice where '{literal}n{literal:#}' is any integer.\
                                             (Ex: {literal}{{_hash:12_}}{literal:#} = 12-character)
  {literal}{{_author:n_}}{literal:#}         The name signature of the individual behind the latest commit.
  {literal}{{_author:e_}}{literal:#}         The email marker boundary of the commit author.
  {literal}{{_time:r_}}{literal:#}           The human-readable relative time interval (e.g., '2 hours ago').
  {literal}{{_time:a_}}{literal:#}           The precise absolute date stamp signature format.
  {literal}{{_dirty_}}{literal:#}            Marker for uncommitted changes.
  {literal}{{_bare_}}{literal:#}             Marker for bare status.
  {literal}{{_contrib:ac_}}{literal:#}       Total number of authors in current branch.
  {literal}{{_contrib:tan_}}{literal:#}      Top author name in current branch.
  {literal}{{_contrib:tae_}}{literal:#}      Top author email in current branch.
  {literal}{{_contrib:tcc_}}{literal:#}      Total number of commits by top author in current branch.
"
    };
}

pub fn print_placeholder_help() {
    let header = CLAP_STYLE.get_header();
    let usage = CLAP_STYLE.get_usage();
    let literal = CLAP_STYLE.get_literal();
    let placeholder = CLAP_STYLE.get_placeholder();

    print!(
        placeholder_template!(),
        header = header,
        usage = usage,
        literal = literal,
        placeholder = placeholder
    );
}
