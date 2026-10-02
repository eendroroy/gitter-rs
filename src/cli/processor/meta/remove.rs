use super::plan::Action;
use super::{fail, load_or_fail, write_or_fail};
use crate::cli::gitter::MetaRemoveArgs;
use crate::meta::{normalize_path, store};

pub fn remove(args: &MetaRemoveArgs) {
    let file = store::resolve(&args.location.directory, &args.location.file.file);
    let mut data = load_or_fail(&file);

    let mut doomed = Vec::new();
    for target in &args.repos {
        let wanted = normalize_path(target);
        let mut matches: Vec<usize> =
            (0..data.repos.len()).filter(|&i| data.repos[i].path == wanted).collect();
        if matches.is_empty() {
            matches = (0..data.repos.len()).filter(|&i| data.repos[i].name() == wanted).collect();
        }
        match matches.as_slice() {
            [] => fail(format!("{} is not in the metafile", target)),
            [i] => doomed.push(*i),
            _ => fail(format!("{} is ambiguous, use the full path", target)),
        }
    }
    doomed.sort_unstable();
    doomed.dedup();

    doomed
        .iter()
        .for_each(|&i| println!("{}", Action::Forget(data.repos[i].clone())));
    if args.dry_run {
        return;
    }

    for &i in doomed.iter().rev() {
        data.repos.remove(i);
    }
    write_or_fail(&file, &data);
}
