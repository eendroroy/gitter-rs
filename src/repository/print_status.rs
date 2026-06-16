use crate::palette::ComponentStyle;
use crate::placeholder::{evaluate_placeholders, replace_placeholders};
use crate::repository::repositories::{Properties, PropertyLengths};
use crate::{STATUS, STYLE};
use std::collections::HashMap;

fn update_evaluation_padding(evaluation: &mut HashMap<String, String>, key: &str, width: usize) {
    if let Some(val) = evaluation.get_mut(key) {
        *val = format!("{:<width$}", val, width = width);
    }
}

fn update_evaluation_style(
    evaluation: &mut HashMap<String, String>,
    key: &str,
    style: ComponentStyle,
) {
    if let Some(val) = evaluation.get_mut(key) {
        *val = style.apply(val);
    }
}

pub fn print_status_line(
    template: Option<String>,
    status: &Properties,
    lengths: Option<PropertyLengths>,
    align: bool,
) {
    println!("{}", get_status_line(template, status, lengths, align));
}

pub fn get_status_line(
    template: Option<String>,
    status: &Properties,
    lengths: Option<PropertyLengths>,
    align: bool,
) -> String {
    let status_template = if let Some(template) = template { template } else { STATUS.to_string() };
    let mut evaluation = evaluate_placeholders(&status_template, status);

    if align && let Some(l) = lengths {
        update_evaluation_padding(&mut evaluation, "{_path:r_}", l.path);
        update_evaluation_padding(&mut evaluation, "{_name_}", l.name);
        update_evaluation_padding(&mut evaluation, "{_branch:n_}", l.branch);
        update_evaluation_padding(&mut evaluation, "{_branch:c_}", l.branch_count);
        update_evaluation_padding(&mut evaluation, "{_commit:c_}", l.commit_count);
        update_evaluation_padding(&mut evaluation, "{_author:n_}", l.author_name);
        update_evaluation_padding(&mut evaluation, "{_author:e_}", l.author_email);
        update_evaluation_padding(&mut evaluation, "{_time:r_}", l.relative_time);
        update_evaluation_padding(&mut evaluation, "{_time:a_}", l.absolute_time);
        update_evaluation_padding(&mut evaluation, "{_bare_}", l.bare);
        update_evaluation_padding(&mut evaluation, "{_contrib:ac_}", l.cs_commit_count);
        update_evaluation_padding(&mut evaluation, "{_contrib:tan_}", l.cs_top_commit_count);
        update_evaluation_padding(&mut evaluation, "{_contrib:tae_}", l.cs_top_author_name);
        update_evaluation_padding(&mut evaluation, "{_contrib:tcc_}", l.cs_top_author_email);
    }

    update_evaluation_style(&mut evaluation, "{_path:r_}", STYLE.path.clone());
    update_evaluation_style(&mut evaluation, "{_name_}", STYLE.name.clone());
    update_evaluation_style(&mut evaluation, "{_branch:n_}", STYLE.branch.clone());
    update_evaluation_style(&mut evaluation, "{_author:n_}", STYLE.author_name.clone());
    update_evaluation_style(&mut evaluation, "{_author:e_}", STYLE.author_email.clone());
    update_evaluation_style(&mut evaluation, "{_time:r_}", STYLE.relative_time.clone());
    update_evaluation_style(&mut evaluation, "{_time:a_}", STYLE.absolute_time.clone());
    update_evaluation_style(&mut evaluation, "{_bare_}", STYLE.bare_style.clone());
    update_evaluation_style(&mut evaluation, "{_contrib:ac_}", STYLE.cs_commit_count.clone());
    update_evaluation_style(&mut evaluation, "{_contrib:tan_}", STYLE.cs_top_commit_count.clone());
    update_evaluation_style(&mut evaluation, "{_contrib:tae_}", STYLE.cs_top_author_name.clone());
    update_evaluation_style(&mut evaluation, "{_contrib:tcc_}", STYLE.cs_top_author_email.clone());

    for (key, val) in evaluation.iter_mut() {
        if key.starts_with("{_hash:") {
            *val = STYLE.commit_hash.apply(val);
        }
    }

    replace_placeholders(&status_template, &evaluation)
}
