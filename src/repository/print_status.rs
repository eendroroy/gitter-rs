use crate::STATUS;
use crate::placeholder::processor::{evaluate_placeholders_styled, replace_placeholders};
use crate::repository::repositories::{Properties, PropertyLengths};

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
    let status_template = template.unwrap_or_else(|| STATUS.to_string());
    let lengths_context = if align { lengths.as_ref() } else { None };
    let evaluation = evaluate_placeholders_styled(&status_template, status, lengths_context);
    replace_placeholders(&status_template, &evaluation)
}
