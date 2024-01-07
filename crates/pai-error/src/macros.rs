#[macro_export]
macro_rules! err {
    ($fmt:expr) => { Err($crate::PError::Info(format!($fmt))) };
    ($fmt:expr, $($args:tt)*) =>{ Err($crate::PError::Info(format!($fmt,$($args)*))) }
}
