extern crate git2;

use git2::{Repository, Status, StatusOptions};

pub fn git_info() -> String {
    let mut repo = match Repository::discover(".") {
        Ok(repo) => repo,
        Err(_) => return String::from(""),
    };
    return format!(
        "[{branch_name}{dirty}]{stashed}{wip} ",
        branch_name = branch_name(&repo),
        dirty = dirty(&repo),
        stashed = stashed(&mut repo),
        wip = wip(&repo)
    );
}

// Handle tags ?
fn branch_name(repo: &Repository) -> String {
    let head = match repo.head() {
        Ok(head) => head,
        Err(_) => return String::from(""),
    };

    let branch_name: String;

    if head.is_branch() {
        branch_name = match head.shorthand() {
            Some(name) => name.to_string(),
            None => return String::from(""),
        };
    } else {
        branch_name = match head.target() {
            Some(commit) => commit.to_string(),
            None => return String::from(""),
        };
    }

    return branch_name;
}

// TODO: This is probably improvable?
fn dirty(repo: &Repository) -> String {
    let mut dirty = "".to_owned();
    let mut added: bool = false;
    let mut modified: bool = false;
    let mut deleted: bool = false;

    let options = &mut StatusOptions::new();
    options.include_ignored(false);
    options.include_untracked(true);

    let statuses = repo.statuses(Some(options)).unwrap();

    for entry in statuses.iter() {
        match entry.status() {
            Status::WT_NEW | Status::INDEX_NEW => added = true,
            Status::WT_MODIFIED | Status::INDEX_MODIFIED => modified = true,
            Status::WT_RENAMED | Status::INDEX_RENAMED => modified = true,
            Status::WT_DELETED | Status::INDEX_DELETED => deleted = true,
            _ => {}
        }
    }
    if added {
        dirty.push_str("+");
    }
    if modified {
        dirty.push_str("#");
    }
    if deleted {
        dirty.push_str("-");
    }

    return dirty;
}

fn stashed(repo: &mut Repository) -> String {
    let mut count = 0;

    match repo.stash_foreach(|_, _, _| {
        count += 1;
        return true;
    }) {
        Ok(()) => {}
        _ => return String::from(""),
    };

    if count == 0 {
        return String::from("");
    }

    return format!("[{}]", count);
}

fn wip(repo: &Repository) -> String {
    let head = match repo.head() {
        Ok(head) => head,
        Err(_) => return String::from(""),
    };

    let commit = match head.peel_to_commit() {
        Ok(c) => c,
        _ => return String::from(""),
    };

    let message = commit.message().unwrap_or("");
    if message.contains("wip") || message.contains("WIP") {
        return "[WIP]".to_string();
    }

    return String::from("");
}
