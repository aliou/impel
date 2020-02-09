extern crate colored;
use colored::*;

mod impel;

fn main() {
    println!(
        // "{hostname}: {cwd} {vcs}{vim}{pchar} ",
        "{hostname}: {cwd} {vcs}{vim}",
        hostname = impel::hostname().color("red").bold(),
        cwd = impel::working_directory(),
        vcs = impel::vcs().color("blue").bold(),
        vim = impel::vim(),
        // pchar = impel::pchar()
    );
}
