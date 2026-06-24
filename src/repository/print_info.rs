use crate::cli::gitter::BoolChoice;
use crate::placeholder::processor::{evaluate_placeholders_styled, replace_placeholders};
use crate::repository::repositories::{Properties, PropertyLengths};

pub fn print_info_line(
    template: &str,
    status: &Properties,
    lengths: Option<PropertyLengths>,
    align: &BoolChoice,
    show_info: &BoolChoice,
) {
    if show_info == &BoolChoice::Always {
        println!("{}", get_info_line(template, status, lengths, align));
    }
}

pub fn get_info_line(
    template: &str,
    status: &Properties,
    lengths: Option<PropertyLengths>,
    align: &BoolChoice,
) -> String {
    let lengths_context = if align == &BoolChoice::Always { lengths.as_ref() } else { None };
    let evaluation = evaluate_placeholders_styled(template, status, lengths_context);
    template
        .split(' ')
        .map(|x| replace_placeholders(x, &evaluation))
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
        .join(" ")
        .replace("\\s", " ")
}
