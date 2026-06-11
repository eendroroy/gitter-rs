use crate::GLOBAL_COLORS;
use crate::placeholder::{evaluate_placeholders, replace_placeholders};
use crate::repository::{Status, StatusLengths};
use colored::Colorize;

pub static DEFAULT_STATUS: &str =
    "{_path:r_} {_name_} {_branch_} {_commit:8_} {_author:e_} {_time:r_}";

pub(crate) fn process_status(
    status: &Status,
    lengths: Option<StatusLengths>,
    align: bool,
) -> String {
    let mut evaluation = evaluate_placeholders(DEFAULT_STATUS.to_string(), status);

    // 2. If length parameters are provided, left-pad the items to ensure alignment
    if align && let Some(l) = lengths {
        if let Some(val) = evaluation.get_mut("{_path:r_}") {
            *val = format!("{:<width$}", val, width = l.path);
        }
        if let Some(val) = evaluation.get_mut("{_name_}") {
            *val = format!("{:<width$}", val, width = l.name);
        }
        if let Some(val) = evaluation.get_mut("{_branch_}") {
            *val = format!("{:<width$}", val, width = l.branch);
        }
        if let Some(val) = evaluation.get_mut("{_author:n_}") {
            *val = format!("{:<width$}", val, width = l.author_name);
        }
        if let Some(val) = evaluation.get_mut("{_author:e_}") {
            *val = format!("{:<width$}", val, width = l.author_email);
        }
        if let Some(val) = evaluation.get_mut("{_time:r_}") {
            *val = format!("{:<width$}", val, width = l.relative_time);
        }
        if let Some(val) = evaluation.get_mut("{_time:d_}") {
            *val = format!("{:<width$}", val, width = l.absolute_time);
        }
    }

    // 3. Apply your color palettes over the padded strings
    if let Some(val) = evaluation.get_mut("{_path:r_}") {
        *val = val.color(GLOBAL_COLORS.path).to_string();
    }
    if let Some(val) = evaluation.get_mut("{_name_}") {
        *val = val.color(GLOBAL_COLORS.name).to_string();
    }
    if let Some(val) = evaluation.get_mut("{_branch_}") {
        *val = val.color(GLOBAL_COLORS.branch).to_string();
    }
    if let Some(val) = evaluation.get_mut("{_commit:f_}") {
        *val = val.color(GLOBAL_COLORS.commit_hash).to_string();
    }
    if let Some(val) = evaluation.get_mut("{_author:n_}") {
        *val = val.color(GLOBAL_COLORS.author_name).to_string();
    }
    if let Some(val) = evaluation.get_mut("{_author:e_}") {
        *val = val.color(GLOBAL_COLORS.author_email).to_string();
    }
    if let Some(val) = evaluation.get_mut("{_time:r_}") {
        *val = val.color(GLOBAL_COLORS.relative_time).to_string();
    }
    if let Some(val) = evaluation.get_mut("{_time:d_}") {
        *val = val.color(GLOBAL_COLORS.absolute_time).to_string();
    }

    // Also color custom variable-width commit tokens (like {_commit:8_}) stored in the map
    for (key, val) in evaluation.iter_mut() {
        if key.starts_with("{_commit:") {
            *val = val.color(GLOBAL_COLORS.commit_hash).to_string();
        }
    }

    // 4. Merge all structural modifications back into your main template sequence
    replace_placeholders(DEFAULT_STATUS.to_string(), evaluation)
}
