#![feature(format_args_nl)]

mod scripts;

use std::env;

use pai_error::PResult;
use scripts::unicode_builder::UnicodeBuilder;

fn main() -> PResult<()> {
    if env!("ENV_AUTOGEN_PROFILE") == "release" {
        UnicodeBuilder::try_new()?.build(&["ID_Start", "ID_Continue"])?;
    }

    Ok(())
}
