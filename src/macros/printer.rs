#[macro_export]
macro_rules! print_error {
    () => {
        println!("{}", *ERROR);
    };
    ($($arg:tt)*) => {
        println!("{}{}", *ERROR, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! print_warn {
    () => {
        println!("{}", *WARN);
    };
    ($($arg:tt)*) => {
        println!("{}{}", *WARN, format_args!($($arg)*));
    };
}
