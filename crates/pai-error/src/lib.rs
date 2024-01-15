use std::{char, io, num};

use syn;
use syn_select;

#[derive(Debug, thiserror::Error)]
pub enum PError {
    #[error(transparent)]
    IO(#[from] io::Error),

    #[error(transparent)]
    ParseInt(#[from] num::ParseIntError),

    #[error(transparent)]
    ParseFloat(#[from] num::ParseFloatError),

    #[error(transparent)]
    CharTryFrom(#[from] char::CharTryFromError),

    #[error(transparent)]
    Syn(#[from] syn::Error),

    #[error(transparent)]
    SynSelect(#[from] syn_select::Error),

    #[error("{0}")]
    Info(String),
}

pub type PResult<T> = Result<T, PError>;
