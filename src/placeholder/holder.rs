use crate::STYLE;
use crate::palette::ComponentStyle;
use crate::repository::repositories::{Properties, PropertyLengths};
use lazy_static::lazy_static;
use regex::Captures;

type ValueFn = Box<dyn Fn(&Properties, Option<&Captures>) -> String + Send + Sync>;
type StatusFn =
    Box<dyn Fn(&Properties, Option<&Captures>, Option<&PropertyLengths>) -> String + Send + Sync>;

pub struct Holder {
    pub tag: String,
    pub value: ValueFn,
    pub status: StatusFn,
}

fn apply_style(value: &str, width: Option<usize>, style: Option<&ComponentStyle>) -> String {
    let val = if let Some(width) = width {
        format!("{:<width$}", value, width = width)
    } else {
        value.to_string()
    };

    if let Some(style) = style { style.apply(val.as_str()) } else { val }
}

macro_rules! create_holder {
    (
        $tag:expr,

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

lazy_static! {
    pub static ref HOLDERS: Vec<Holder> = vec![
        create_holder!("name", |s, _c| s.name.clone(), |s, _c, l| apply_style(
            &s.name,
            l.map(|i| i.name),
            Some(&STYLE.name)
        )),
        create_holder!("path:r", |s, _c| s.relative_path.clone(), |s, _c, l| apply_style(
            &s.relative_path,
            l.map(|i| i.relative_path),
            Some(&STYLE.path)
        )),
        create_holder!("path:a", |s, _c| s.absolute_path.clone(), |s, _c, l| apply_style(
            &s.absolute_path,
            l.map(|i| i.absolute_path),
            Some(&STYLE.path)
        )),
        create_holder!("branch:n", |s, _c| s.branch.clone(), |s, _c, l| apply_style(
            &s.branch,
            l.map(|i| i.branch),
            Some(&STYLE.branch)
        )),
        create_holder!("branch:c", |s, _c| s.branch_count.to_string(), |s, _c, l| apply_style(
            &s.branch_count.to_string(),
            l.map(|i| i.branch_count),
            Some(&STYLE.branch)
        )),
        create_holder!("hash:f", |s, _c| s.commit_hash.clone(), |s, _c, _l| apply_style(
            &s.commit_hash,
            None,
            Some(&STYLE.commit_hash)
        )),
        create_holder!("commit:c", |s, _c| s.commit_count.to_string(), |s, _c, l| apply_style(
            &s.commit_count.to_string(),
            l.map(|i| i.commit_count),
            Some(&STYLE.commit_hash),
        )),
        create_holder!("author:e", |s, _c| s.author_email.clone(), |s, _c, l| apply_style(
            &s.author_email,
            l.map(|i| i.author_email),
            Some(&STYLE.author_email)
        )),
        create_holder!("author:n", |s, _c| s.author_name.clone(), |s, _c, l| apply_style(
            &s.author_name,
            l.map(|i| i.author_name),
            Some(&STYLE.author_name)
        )),
        create_holder!("time:r", |s, _c| s.relative_time.clone(), |s, _c, l| apply_style(
            &s.relative_time,
            l.map(|i| i.relative_time),
            Some(&STYLE.relative_time)
        )),
        create_holder!("time:a", |s, _c| s.absolute_time.clone(), |s, _c, l| apply_style(
            &s.absolute_time,
            l.map(|i| i.absolute_time),
            Some(&STYLE.absolute_time)
        )),
        create_holder!("dirty", |s, _c| s.dirty.clone(), |s, _c, _l| apply_style(
            &s.dirty, None, None
        )),
        create_holder!("bare", |s, _c| s.bare.clone(), |s, _c, l| apply_style(
            &s.bare,
            l.map(|i| i.bare),
            Some(&STYLE.bare_style)
        )),
        create_holder!("contrib:ac", |s, _c| s.cs.author_count.to_string(), |s, _c, l| {
            apply_style(
                &s.cs.author_count.to_string(),
                l.map(|i| i.cs_author_count),
                Some(&STYLE.cs_author_count),
            )
        }),
        create_holder!("contrib:tan", |s, _c| s.cs.top_author_name.to_string(), |s, _c, l| {
            apply_style(
                &s.cs.top_author_name.to_string(),
                l.map(|i| i.cs_top_author_name),
                Some(&STYLE.cs_top_author_name),
            )
        }),
        create_holder!("contrib:tae", |s, _c| s.cs.top_author_email.to_string(), |s, _c, l| {
            apply_style(
                &s.cs.top_author_email.to_string(),
                l.map(|i| i.cs_top_author_email),
                Some(&STYLE.cs_top_author_email),
            )
        }),
        create_holder!("contrib:tcc", |s, _c| s.cs.top_commit_count.to_string(), |s, _c, l| {
            apply_style(
                &s.cs.top_commit_count.to_string(),
                l.map(|i| i.cs_top_commit_count),
                Some(&STYLE.cs_top_commit_count),
            )
        }),
        create_holder!(
            "hash",
            |s, caps| {
                if let Some(c) = caps
                    && let Some(len_match) = c.get(2)
                    && let Ok(req_len) = len_match.as_str().parse::<usize>()
                {
                    let target_len = std::cmp::min(req_len, s.commit_hash.len());
                    return s.commit_hash[..target_len].to_string();
                }
                s.commit_hash.clone()
            },
            |s, caps, _l| {
                let target_len = caps
                    .and_then(|c| c.get(2))
                    .and_then(|m| m.as_str().parse::<usize>().ok())
                    .map(|len| std::cmp::min(len, s.commit_hash.len()))
                    .unwrap_or(s.commit_hash.len());

                apply_style(&s.commit_hash[..target_len], None, Some(&STYLE.commit_hash))
            }
        ),
    ];
}
