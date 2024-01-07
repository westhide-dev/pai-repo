use std::{
    env,
    fmt::{self, Display},
    fs,
    io::{self, ErrorKind::Interrupted},
    iter::FromIterator,
    path::PathBuf,
    process::{Command, Output},
};

const PROPERTIES: &[&str] = &["ID_Start", "ID_Continue"];

const DOWNLOAD_DIR: &str = env!("ENV_PAI_UNICODE_DOWNLOAD_DIR");
