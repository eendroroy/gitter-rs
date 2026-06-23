#[macro_export]
macro_rules! print_error {
    () => {
        use crate::style::ERROR;
        println!("{}", *ERROR);
    };
    ($($arg:tt)*) => {
        use crate::style::ERROR;
        println!("{}{}", *ERROR, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! print_warn {
    () => {
        use crate::style::WARN;
        println!("{}", *WARN);
    };
    ($($arg:tt)*) => {
        use crate::style::WARN;
        println!("{}{}", *WARN, format_args!($($arg)*));
    };
}
