use crate::REPO_INFO;
use crate::placeholder::processor::{evaluate_placeholders_styled, replace_placeholders};
use crate::repository::repositories::{Properties, PropertyLengths};

pub fn print_info_line(
    template: Option<String>,
    status: &Properties,
    lengths: Option<PropertyLengths>,
    align: bool,
) {
    println!("{}", get_info_line(template, status, lengths, align));
}

pub fn get_info_line(
    template: Option<String>,
    status: &Properties,
    lengths: Option<PropertyLengths>,
    align: bool,
) -> String {
    let status_template = template.unwrap_or_else(|| REPO_INFO.to_string());
    let lengths_context = if align { lengths.as_ref() } else { None };
    let evaluation = evaluate_placeholders_styled(&status_template, status, lengths_context);
    status_template
        .split(' ')
        .map(|x| replace_placeholders(x, &evaluation))
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
        .join(" ")
        .replace("\\s", " ")
}
