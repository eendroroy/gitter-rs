use crate::repository::repositories::Properties;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::HashMap;

lazy_static! {
    static ref PLACEHOLDER_RE: Regex = Regex::new(r"\{_([a-z:]+?)(?::(\d+))?_\}").unwrap();
}

pub fn evaluate_placeholders(base_string: &str, status: &Properties) -> HashMap<String, String> {
    let mut evaluation = HashMap::new();

    for caps in PLACEHOLDER_RE.captures_iter(base_string) {
        let full_tag = caps.get(0).unwrap().as_str();

        if evaluation.contains_key(full_tag) {
            continue;
        }

        let key = caps.get(1).unwrap().as_str();
        let value = match key {
            "name" => status.name.clone(),
            "path:r" => status.relative_path.clone(),
            "path:a" => status.absolute_path.clone(),
            "branch:n" => status.branch.clone(),
            "branch:c" => status.branch_count.to_string(),
            "hash:f" => status.commit_hash.clone(),
            "commit:c" => status.commit_count.to_string(),
            "author:e" => status.author_email.clone(),
            "author:n" => status.author_name.clone(),
            "time:r" => status.relative_time.clone(),
            "time:a" => status.absolute_time.clone(),
            "dirty" => status.dirty.clone(),
            "bare" => status.bare.clone(),
            "contrib:ac" => status.contribution_summary.author_count.to_string(),
            "contrib:tan" => status.contribution_summary.top_author_name.to_string(),
            "contrib:tae" => status.contribution_summary.top_author_email.to_string(),
            "contrib:tcc" => status.contribution_summary.top_commit_count.to_string(),

            "hash" => {
                if let Some(len_match) = caps.get(2) {
                    if let Ok(req_len) = len_match.as_str().parse::<usize>() {
                        let target_len = std::cmp::min(req_len, status.commit_hash.len());
                        status.commit_hash[..target_len].to_string()
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            _ => continue,
        };

        evaluation.insert(full_tag.to_string(), value);
    }

    evaluation
}

pub fn replace_placeholders(base_string: &str, evaluation: &HashMap<String, String>) -> String {
    PLACEHOLDER_RE
        .replace_all(base_string, |caps: &Captures| {
            let full_tag = caps.get(0).unwrap().as_str();

            match evaluation.get(full_tag) {
                Some(evaluated_value) => evaluated_value.clone(),
                None => full_tag.to_string(),
            }
        })
        .into_owned()
}
