use chrono::{Duration, Utc};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn run_provision() {
    let local_dir = PathBuf::from(".local");
    fs::create_dir_all(&local_dir).expect("Failed to create .local directory");

    let base_path = fs::canonicalize(&local_dir).expect("Failed to get absolute path for .local");

    for i in 0..=7 {
        let days_ago = (i + 1) as i64;
        let time_stamp =
            (Utc::now() - Duration::days(days_ago)).format("%Y-%m-%dT%H:%M:%S").to_string();

        let repo_dir = base_path.join(format!("repo_0{}", i));
        let bare_dir = base_path.join(format!("repo_bare_0{}", i));

        if repo_dir.exists() {
            println!("{} already exists. Deleting....", repo_dir.display());
            fs::remove_dir_all(&repo_dir).expect("Failed to delete existing repo_dir");
        }

        fs::create_dir_all(&repo_dir).expect("Failed to create repo directory");
        Command::new("git")
            .args(["-C", repo_dir.to_str().unwrap(), "init", "-b", "master"])
            .output()
            .expect("Failed to run git init");

        let make_first_commit = || {
            let file_path = repo_dir.join("file");
            let file_contents = format!("{}/file-1\n", repo_dir.display());
            fs::write(&file_path, file_contents).expect("Failed to write file");

            Command::new("git")
                .args(["-C", repo_dir.to_str().unwrap(), "add", "."])
                .output()
                .expect("Failed git add");

            Command::new("git")
                .args(["-C", repo_dir.to_str().unwrap(), "commit", "-m", "first commit"])
                .env("GIT_AUTHOR_DATE", &time_stamp)
                .env("GIT_COMMITTER_DATE", &time_stamp)
                .output()
                .expect("Failed git commit");
        };

        let clone_bare_repo = || {
            if bare_dir.exists() {
                println!("{} already exists. Deleting....", bare_dir.display());
                fs::remove_dir_all(&bare_dir).expect("Failed to delete existing bare_dir");
            }
            Command::new("git")
                .args(["clone", "--bare", repo_dir.to_str().unwrap(), bare_dir.to_str().unwrap()])
                .output()
                .expect("Failed to clone bare repo");
        };

        match i {
            0 => {
                make_first_commit();
                clone_bare_repo();
            }
            1 | 2 => {
                make_first_commit();
            }
            3 | 4 | 5 => {
                make_first_commit();
                Command::new("git")
                    .args([
                        "-C",
                        repo_dir.to_str().unwrap(),
                        "checkout",
                        "-b",
                        &format!("feature/feature-{}", i),
                    ])
                    .output()
                    .expect("Failed to checkout feature branch");
            }
            6 => {
                make_first_commit();

                let file2_path = repo_dir.join("file-2");
                let file2_contents = format!("{}/file-2\n", repo_dir.display());
                fs::write(&file2_path, file2_contents).expect("Failed to write file-2");

                Command::new("git")
                    .args(["-C", repo_dir.to_str().unwrap(), "add", "."])
                    .output()
                    .expect("Failed git add for file-2");

                Command::new("git")
                    .args(["-C", repo_dir.to_str().unwrap(), "commit", "-m", "second commit"])
                    .env("GIT_AUTHOR_DATE", &time_stamp)
                    .env("GIT_COMMITTER_DATE", &time_stamp)
                    .output()
                    .expect("Failed git commit for second commit");

                let rev_output = Command::new("git")
                    .args(["-C", repo_dir.to_str().unwrap(), "rev-list", "--max-parents=0", "HEAD"])
                    .output()
                    .expect("Failed to run git rev-list");

                let root_commit = String::from_utf8_lossy(&rev_output.stdout).trim().to_string();

                Command::new("git")
                    .args(["-C", repo_dir.to_str().unwrap(), "checkout", &root_commit])
                    .output()
                    .expect("Failed to checkout root commit");

                clone_bare_repo();
                clone_bare_repo();
            }
            _ => {}
        }
    }

    let ignore_path = base_path.join(".gitterignore");

    if ignore_path.exists() {
        println!("{} already exists. Deleting....", ignore_path.display());
        fs::remove_file(&ignore_path).expect("Failed to delete existing .gitterignore file");
    }
    fs::write(&ignore_path, "repo_01\n").expect("Failed to create and write to .gitterignore file");
}
