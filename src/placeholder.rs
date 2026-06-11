use crate::repository::Status;

pub(crate) fn evaluate_placeholders(mut base_string: String, status: &Status) -> String {
    base_string = base_string.replace("{_name_}", status.name.as_str());
    base_string = base_string.replace("{_path:r_}", status.path.as_str());
    base_string = base_string.replace("{_path:a_}", status.absolute_path.as_str());
    base_string = base_string.replace("{_branch_}", status.branch.as_str());
    // base_string = base_string.replace("{_commit:f_}", );
    // base_string = base_string.replace("{_commit:<n>_}", );
    // base_string = base_string.replace("{_commit:c_}", );
    base_string = base_string.replace("{_author:e_}", status.author_email.as_str());
    base_string = base_string.replace("{_author:n_}", status.author_name.as_str());
    base_string = base_string.replace("{_time:r_}", status.relative_time.as_str());
    base_string = base_string.replace("{_time:d_}", status.absolute_time.as_str());

    base_string
}
