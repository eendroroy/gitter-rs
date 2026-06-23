#[macro_export]
macro_rules! print_error {
    () => {
        use $crate::style::ERROR;
        eprintln!("{}", *ERROR);
    };
    ($($arg:tt)*) => {
        use $crate::style::ERROR;
        eprintln!("{}{}", *ERROR, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! print_warn {
    () => {c
        use $crate::style::WARN;
        eprintln!("{}", *WARN);
    };
    ($($arg:tt)*) => {
        use $crate::style::WARN;
        eprintln!("{}{}", *WARN, format_args!($($arg)*));
    };
}
