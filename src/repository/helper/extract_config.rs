use git2::Config;

pub fn extract_config(config: &Option<Config>, property: &str) -> String {
    config.as_ref().and_then(|c| c.get_string(property).ok()).unwrap_or_default()
}
