use crate::STYLE;
use crate::repository::helper::DETACHED;
use crate::repository::repositories::{Properties, PropertyLengths};
use crate::style::ComponentStyle;
use lazy_static::lazy_static;
use regex::Captures;
use std::cmp::min;

type ValueFn = Box<dyn Fn(&Properties, Option<&Captures>) -> String + Send + Sync>;
type StatusFn =
    Box<dyn Fn(&Properties, Option<&Captures>, Option<&PropertyLengths>) -> String + Send + Sync>;

pub struct Holder {
    pub tag: String,
    pub value: ValueFn,
    pub status: StatusFn,
}

fn apply_style(value: &str, width: Option<usize>, style: Option<&ComponentStyle>) -> String {
    let val = match width {
        Some(width) => format!("{:<width$}", value, width = width),
        None => value.to_string(),
    };

    if val.trim().is_empty() {
        return val.to_string();
    }

    match style {
        Some(style) => style.apply(val.as_str()),
        None => val,
    }
}

macro_rules! create_holder {
    (
        $tag:expr,
        $full_tag:expr,
        $desc:expr,

        |$props_v:ident, $idx_v:ident| $value_body:expr,
        |$props_s:ident, $idx_s:ident, $lens_s:ident| $status_body:expr $(,)?
    ) => {
        Holder {
            tag: $tag.to_string(),
            value: Box::new(move |$props_v, $idx_v| $value_body),
            status: Box::new(move |$props_s, $idx_s, $lens_s| $status_body),
        }
    };
}

macro_rules! define_holders {
    (
        $(
            {
                $tag:literal, $full_tag:literal, $desc:literal,

                |$props_v:ident, $idx_v:ident| $value_body:expr,
                |$props_s:ident, $idx_s:ident, $lens_s:ident| $status_body:expr $(,)?
            }
        )*
    ) => {
        lazy_static! {
            pub static ref HOLDERS: Vec<Holder> = vec![
                $(
                    create_holder!(
                        $tag,
                        $full_tag,
                        $desc,

                        |$props_v, $idx_v| $value_body,
                        |$props_s, $idx_s, $lens_s| $status_body
                    ),
                )*
            ];
        }

        #[macro_export]
        macro_rules! placeholder_template {
            ($header:expr, $usage:expr, $literal:expr, $placeholder:expr, $tag_width:expr) => {{
                print!(
                    concat!(
                        "{header}Gitter Template Placeholders{header:#}\n\n",
                        "{usage}Usage:{usage:#}\n",
                        "  Pass these tags within string sequences to dynamically extract local repository data.\n",
                        "  Example: {literal}gitter bash{literal:#} {placeholder}-- echo \"Current branch name is: {{_branch:n_}}\"{placeholder:#}\n\n",
                        "{header}Available Placeholders:{header:#}\n"
                    ),
                    header = $header,
                    usage = $usage,
                    literal = $literal,
                    placeholder = $placeholder
                );

                $(
                    let formatted_desc = $desc
                        .replace("{literal}", &$literal.to_string())
                        .replace("{literal:#}", "\x1b[0m");

                    println!(
                        "  {literal}{tag_val:<width$}{literal:#} {desc}",
                        literal = $literal,
                        tag_val = $full_tag,
                        width = $tag_width,
                        desc = formatted_desc
                    );
                )*
            }};
        }
    };
}

define_holders! {
    {
        "remote:n", "{_remote:n_}", "Name of the remote",

        |s, _c| s.remote_name.clone(),
        |s, _c, l| apply_style(&s.remote_name, l.map(|i| i.remote_name), Some(&STYLE.remote_name))
    }
    {
        "remote:f", "{_remote:f_}", "Remote url (fetch url)",

        |s, _c| s.remote_fetch.clone(),
        |s, _c, l| apply_style(&s.remote_fetch, l.map(|i| i.remote_fetch), Some(&STYLE.remote_fetch))
    }
    {
        "remote:p", "{_remote:p_}", "Remote push url",

        |s, _c| s.remote_push.clone(),
        |s, _c, l| apply_style(&s.remote_push, l.map(|i| i.remote_push), Some(&STYLE.remote_push))
    }
    {
        "name", "{_name_}", "The simple name of the repository directory.",

        |s, _c| s.name.clone(),
        |s, _c, l| apply_style(&s.name, l.map(|i| i.name), Some(&STYLE.name))
    }

    {
        "path:r", "{_path:r_}", "The relative path from your execution context.",

        |s, _c| s.relative_path.clone(),
        |s, _c, l| apply_style(&s.relative_path, l.map(|i| i.relative_path), Some(&STYLE.path))
    }

    {
        "path:a", "{_path:a_}", "The complete absolute file path on the system filesystem.",

        |s, _c| s.absolute_path.clone(),
        |s, _c, l| apply_style(&s.absolute_path, l.map(|i| i.absolute_path), Some(&STYLE.path))
    }

    {
        "branch:n", "{_branch:n_}", "The active checked-out Git branch head.",

        |s, _c| s.branch.clone(),
        |s, _c, l| {
            if s.branch == DETACHED {
                apply_style(&s.branch, l.map(|i| i.branch), Some(&STYLE.detached))
            } else {
                apply_style(&s.branch, l.map(|i| i.branch), Some(&STYLE.branch))
            }
        }
    }

    {
        "branch:c", "{_branch:c_}", "Total number of branches.",

        |s, _c| s.branch_count.to_string(),
        |s, _c, l| apply_style(&s.branch_count.to_string(), l.map(|i| i.branch_count), Some(&STYLE.branch))
    }

    {
        "hash:f", "{_hash:f_}", "The full 40-character Git commit hash string.",

        |s, _c| s.commit_hash.clone(),
        |s, _c, _l| apply_style(&s.commit_hash, None, Some(&STYLE.commit_hash))
    }

    {
        "hash", "{_hash:<n>_}", "A variable length commit SHA slice where 'n' is any integer. Ex: {literal}{_hash:12_}{literal:#} = 12-characters)",

        |s, caps| {
            if let Some(c) = caps
                && let Some(len_match) = c.get(2)
                && let Ok(req_len) = len_match.as_str().parse::<usize>()
            {
                let target_len = min(req_len, s.commit_hash.len());
                return s.commit_hash[..target_len].to_string();
            }
            s.commit_hash.clone()
        },

        |s, caps, l| {
            if let Some(c) = caps
                && let Some(len_match) = c.get(2)
                && let Ok(req_len) = len_match.as_str().parse::<usize>()
            {
                let commit_len = min(req_len, s.commit_hash.len());
                let target_len = min(req_len, l.map(|i| i.commit_hash).unwrap_or(0));
                apply_style(&s.commit_hash[..commit_len], Some(target_len), Some(&STYLE.commit_hash))
            } else {
                apply_style(&s.commit_hash, None, Some(&STYLE.commit_hash))
            }
        }
    }

    {
        "commit:c", "{_commit:c_}", "Total number of commits in current branch.",

        |s, _c| s.commit_count.to_string(),
        |s, _c, l| apply_style(&s.commit_count.to_string(), l.map(|i| i.commit_count), Some(&STYLE.commit_hash))
    }

    {
        "author:e", "{_author:e_}", "The email marker boundary of the commit author.",

        |s, _c| s.author_email.clone(),
        |s, _c, l| apply_style(&s.author_email, l.map(|i| i.author_email), Some(&STYLE.author_email))
    }

    {
        "author:n", "{_author:n_}", "The name signature of the individual behind the latest commit.",

        |s, _c| s.author_name.clone(),
        |s, _c, l| apply_style(&s.author_name, l.map(|i| i.author_name), Some(&STYLE.author_name))
    }

    {
        "time:r", "{_time:r_}", "The human-readable relative time interval (e.g., '2 hours ago').",

        |s, _c| s.relative_time.clone(),
        |s, _c, l| apply_style(&s.relative_time, l.map(|i| i.relative_time), Some(&STYLE.relative_time))
    }

    {
        "time:a", "{_time:a_}", "The precise absolute date stamp signature format.",

        |s, _c| s.absolute_time.clone(),
        |s, _c, l| apply_style(&s.absolute_time, l.map(|i| i.absolute_time), Some(&STYLE.absolute_time))
    }

    {
        "dirty", "{_dirty_}", "Marker for uncommitted changes.",

        |s, _c| s.dirty.clone(),
        |s, _c, _l| {
            match s.dirty.as_str() {
                "DIRTY" => apply_style("DIRTY", None, Some(&STYLE.dirty_style)),
                "CLEAN" => apply_style("CLEAN", None, Some(&STYLE.clean_style)),
                _ => "".to_string()
            }
        }
    }

    {
        "bare", "{_bare_}", "Marker for bare status.",

        |s, _c| s.bare.clone(),
        |s, _c, l| apply_style(&s.bare, l.map(|i| i.bare), Some(&STYLE.bare_style))
    }

    {
        "size", "{_size_}", "Size of the repository",

        |s, _c| s.repo_size.to_string(),
        |s, _c, l| apply_style(&s.repo_size.to_string(), l.map(|i| i.repo_size), Some(&STYLE.repo_size))
    }

    {
        "language", "{_language_}", "Top language used by the repository",

        |s, _c| s.top_lang.to_string(),
        |s, _c, l| apply_style(&s.top_lang.to_string(), l.map(|i| i.top_lang), Some(&STYLE.top_lang))
    }
}
