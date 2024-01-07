use std::{env, io};

mod scripts;

use scripts::UnicodeBuilder;

fn main() -> io::Result<()> {
    let Ok(profile) = env::var("PROFILE") else {
        panic!("env PROFILE Not found")
    };

    if profile == "release" {
        UnicodeBuilder::generate_tables()?;
    }

    Ok(())
}
