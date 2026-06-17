use crate::placeholder::HOLDERS;
use crate::repository::repositories::{Properties, PropertyLengths};
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

        if let Some(holder) = HOLDERS.iter().find(|h| h.tag == key) {
            let value = (holder.value)(status, Some(&caps));
            evaluation.insert(full_tag.to_string(), value);
        }
    }

    evaluation
}

pub fn evaluate_placeholders_styled(
    base_string: &str,
    status: &Properties,
    lengths: Option<&PropertyLengths>, // Added parameter to forward to HOLDERS
) -> HashMap<String, String> {
    let mut evaluation = HashMap::new();

    for caps in PLACEHOLDER_RE.captures_iter(base_string) {
        let full_tag = caps.get(0).unwrap().as_str();
        if evaluation.contains_key(full_tag) {
            continue;
        }

        let key = caps.get(1).unwrap().as_str();

        if let Some(holder) = HOLDERS.iter().find(|h| h.tag == key) {
            // Passes status, captures, and lengths context down to your holders
            let value = (holder.status)(status, Some(&caps), lengths);
            evaluation.insert(full_tag.to_string(), value);
        }
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
