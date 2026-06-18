use crate::IGNORE_FILE;
use crate::gitter::CLAP_STYLE;

macro_rules! gitterignore_template {
    () => {
"\
{header}Gitterignore File Format{header:#}

{usage}Description:{usage:#}
  The {literal}{IGNORE_FILE}{literal:#} file specifies repositories to be ignored by gitter.
  Each line is a pattern. Empty lines and lines starting with {literal}#{literal:#} are comments.
  All path should be relative to the {IGNORE_FILE} file.

{header}Patterns:{header:#}
  Patterns are interpreted based on their format:

  {literal}path/to/repo{literal:#}  - Ignores repositories with the exact relative path or name.
  {literal}prefix*{literal:#}       - Ignores repositories whose relative path or name starts with a given prefix.
  {literal}dir_name/*{literal:#}    - Ignores repositories if their immediate sub-directory is an exact name.
  {literal}dir_prefix*/*{literal:#} - Ignores repositories if their immediate sub-directory starts with a given prefix.
"
    };
}

pub fn print_gitterignore_help() {
    let header = CLAP_STYLE.get_header();
    let usage = CLAP_STYLE.get_usage();
    let literal = CLAP_STYLE.get_literal();

    print!(
        gitterignore_template!(),
        header = header,
        usage = usage,
        literal = literal,
        IGNORE_FILE = IGNORE_FILE
    );
}
