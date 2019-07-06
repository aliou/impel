extern crate colored;
use colored::*;

mod impel;

fn main() {
    println!("{}", impel())
}

fn impel() -> String {
    return format!(
        "{hostname}: {cwd} {vcs}{vim}{pchar} ",
        hostname = impel::hostname().color("red").bold(),
        cwd = impel::working_directory(),
        vcs = impel::vcs().color("blue").bold(),
        vim = impel::vim(),
        pchar = impel::pchar()
    );
}
