extern crate hostname;

use std::env;
use std::path::Path;

mod vcs;

pub fn hostname() -> String {
    return match hostname::get_hostname() {
        None => String::from(""),
        Some(hs) => hs,
    };
}

pub fn working_directory() -> String {
    let full_path = format!("{}", env::current_dir().unwrap_or("".into()).display());
    let path = full_path.split("/").last().unwrap_or("").to_string();

    return path;
}

pub fn pchar() -> String {
    return String::from("›");
}

pub fn vcs() -> String {
    return vcs::git_info();
}

pub fn vim() -> String {
    return if Path::new("Session.vim").exists() {
        String::from("[$] ")
    } else {
        String::from("")
    };
}
