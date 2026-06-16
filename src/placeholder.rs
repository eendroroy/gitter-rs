use crate::repository::repositories::Properties;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub fn evaluate_placeholders(base_string: String, status: &Properties) -> HashMap<String, String> {
    let mut evaluation = HashMap::new();

    if base_string.contains("{_name_}") {
        evaluation.insert("{_name_}".to_string(), status.name.clone());
    }
    if base_string.contains("{_path:r_}") {
        evaluation.insert("{_path:r_}".to_string(), status.relative_path.clone());
    }
    if base_string.contains("{_path:a_}") {
        evaluation.insert("{_path:a_}".to_string(), status.absolute_path.clone());
    }
    if base_string.contains("{_branch:n_}") {
        evaluation.insert("{_branch:n_}".to_string(), status.branch.clone());
    }
    if base_string.contains("{_branch:c_}") {
        evaluation.insert("{_branch:c_}".to_string(), status.branch_count.to_string());
    }
    if base_string.contains("{_commit:f_}") {
        evaluation.insert("{_commit:f_}".to_string(), status.commit_hash.clone());
    }
    if base_string.contains("{_commit:c_}") {
        evaluation.insert("{_commit:c_}".to_string(), status.commit_count.to_string());
    }
    if base_string.contains("{_author:e_}") {
        evaluation.insert("{_author:e_}".to_string(), status.author_email.clone());
    }
    if base_string.contains("{_author:n_}") {
        evaluation.insert("{_author:n_}".to_string(), status.author_name.clone());
    }
    if base_string.contains("{_time:r_}") {
        evaluation.insert("{_time:r_}".to_string(), status.relative_time.clone());
    }
    if base_string.contains("{_time:d_}") {
        evaluation.insert("{_time:d_}".to_string(), status.absolute_time.clone());
    }
    if base_string.contains("{_dirty_}") {
        evaluation.insert(
            "{_dirty_}".to_string(),
            if status.is_dirty { "DIRTY" } else { "CLEAN" }.to_string(),
        );
    }

    lazy_static! {
        static ref RE: Regex = Regex::new(r"\{_commit:(\d+)_\}").unwrap();
    }
    for caps in RE.captures_iter(&base_string) {
        let full_match = caps.get(0).unwrap().as_str();

        if let Some(num_str) = caps.get(1)
            && let Ok(requested_len) = num_str.as_str().parse::<usize>()
        {
            let full_hash = &status.commit_hash;
            let target_len = std::cmp::min(requested_len, full_hash.len());
            let sliced_hash = full_hash[..target_len].to_string();
            evaluation.insert(full_match.to_string(), sliced_hash);
        }
    }

    evaluation
}

pub fn replace_placeholders(
    mut base_string: String,
    evaluation: HashMap<String, String>,
) -> String {
    for (placeholder_tag, evaluated_value) in evaluation {
        base_string = base_string.replace(&placeholder_tag, &evaluated_value);
    }

    base_string
}
