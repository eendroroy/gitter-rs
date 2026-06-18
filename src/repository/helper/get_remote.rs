use git2::Repository;

// (name, url, push_url)
pub fn get_remote(repository: &Repository) -> (String, String, String) {
    if let Some(remotes) = repository.remotes().iter().next()
        && let Some(Ok(Some(remote_name))) = remotes.iter().next()
    {
        let remote = repository.find_remote(remote_name).unwrap();
        let fetch_url = remote.url().unwrap_or("");
        let push_url = if let Ok(Some(push_url)) = remote.pushurl() {
            push_url
        } else {
            fetch_url
        };
        return (remote_name.trim().to_string(), fetch_url.to_string(), push_url.to_string());
    }

    ("".to_string(), "".to_string(), "".to_string())
}
