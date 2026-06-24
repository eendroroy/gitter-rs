use crate::cli::gitter::CLAP_STYLE;

macro_rules! help_template {
    () => {
"\
{header}Filter Syntax Help{header:#}

{usage}Description:{usage:#}
  Filter expressions allow you to select repositories based on various criteria.
  You can combine multiple filter clauses using boolean logic.

{usage}General Syntax:{usage:#}
  {literal}<filter_clause> && <filter_clause>{literal:#}   - Combines expressions with logical AND.
  {literal}<filter_clause> || <filter_clause>{literal:#}   - Combines expressions with logical OR.
  {literal}! <filter_clause>{literal:#}                    - Negates a filter clause or an expression.
  {literal}(<expression>){literal:#}                       - Groups expressions.

  Example: {literal}name:project+ && (branch:dev || branch:feature+){literal:#}

{usage}Filter Clause Format:{usage:#}
  A filter clause follows the format: {literal}[!] <prefix>:<value_pattern>{literal:#}

{header}Prefixes:{header:#}
  - {literal}path{literal:#}     Filters by the relative path of the repository.
  - {literal}name{literal:#}     Filters by the name of the repository.
  - {literal}branch{literal:#}   Filters by the current branch name.
  - {literal}dirty{literal:#}    Filters by uncommitted changes. (Takes no value. ie. - {literal}dirty:{literal:#})
  - {literal}bare{literal:#}     Filters by bare status. (Takes no value. ie. - {literal}bare:{literal:#})
  - {literal}language{literal:#} Filters by top language used.
  - {literal}active{literal:#}   Filters by the age of the last commit.(See: {usage}Active Filter Value Patterns{usage:#})

{header}Value Patterns:{header:#}
  - {literal}value{literal:#}    Exact match.
  - {literal}value+{literal:#}   Starts with {literal}value{literal:#} (prefix match).
  - {literal}+value{literal:#}   Ends with {literal}value{literal:#} (suffix match).
  - {literal}+value+{literal:#}  Contains {literal}value{literal:#} (substring match).

{usage}Active Filter Value Patterns:{usage:#}
  These patterns are used with the {literal}active{literal:#} prefix to filter by commit age.
  Duration units: {literal}d{literal:#} (days), {literal}h{literal:#} (hours), {literal}m{literal:#} (minutes), {literal}w{literal:#} (weeks), {literal}M{literal:#} (months), {literal}y{literal:#} (years).
  Units can be combined (e.g., {literal}1y3M2d{literal:#}).

  - {literal}<duration{literal:#}  Last commit was less than the specified duration ago. Ex: {literal}active:<7d{literal:#} (last commit within the last 7 days).
  - {literal}>duration{literal:#}  Last commit was more than the specified duration ago. Ex: {literal}active:>1M{literal:#} (last commit older than 1 month).
  - {literal}duration{literal:#}   Last commit was approximately the specified duration ago. Ex: {literal}active:2w{literal:#} (last commit around 2 weeks ago).

{header}Examples:{header:#}
  - {literal}name:my-repo{literal:#}                 Matches repositories with the exact name {literal}my-repo{literal:#}.
  - {literal}path:src/projects+{literal:#}           Matches repositories whose relative path starts with {literal}src/projects{literal:#}.
  - {literal}branch:+main{literal:#}                 Matches repositories whose current branch ends with {literal}main{literal:#}.
  - {literal}name:+feature+{literal:#}               Matches repositories whose name contains {literal}feature{literal:#}.
  - {literal}active:<7d{literal:#}                   Matches repositories with a last commit within the last 7 days.
  - {literal}active:>1y3M{literal:#}                 Matches repositories with a last commit older than 1 year and 3 months.
  - {literal}name:repo1 || name:repo2{literal:#}     Matches repositories named {literal}repo1{literal:#} OR {literal}repo2{literal:#}.
  - {literal}name:project+ && branch:main{literal:#} Matches repositories whose name starts with {literal}project{literal:#} AND are on the {literal}main{literal:#} branch.
  - {literal}!branch:main{literal:#}                 Matches repositories that are NOT on the {literal}main{literal:#} branch.
  - {literal}!(name:test+ || name:temp+){literal:#}  Matches repositories whose name does NOT start with {literal}test{literal:#} OR {literal}temp{literal:#}.
  - {literal}path:backend+ && (branch:dev || branch:feature+){literal:#}
                                 Matches backend repositories that are on the {literal}dev{literal:#} branch OR a branch starting with {literal}feature{literal:#}.
"
    };
}

pub fn print_filter_help() {
    let header = CLAP_STYLE.get_header();
    let usage = CLAP_STYLE.get_usage();
    let literal = CLAP_STYLE.get_literal();

    print!(help_template!(), header = header, usage = usage, literal = literal);
}
