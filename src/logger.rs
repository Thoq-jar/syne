pub const GRAY: &str = "\x1b[90m";
pub const PURPLE: &str = "\x1b[35m";
pub const YELLOW: &str = "\x1b[33m";
pub const RED: &str = "\x1b[31m";
pub const RESET: &str = "\x1b[0m";

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("{}[{}{}INFO{}{}]{} {}",
            $crate::logger::GRAY,
            $crate::logger::RESET,
            $crate::logger::PURPLE,
            $crate::logger::RESET,
            $crate::logger::GRAY,
            $crate::logger::RESET,
            format!($($arg)*)
        );
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        println!("{}[{}{}WARN{}{}]{} {}",
            $crate::logger::GRAY,
            $crate::logger::RESET,
            $crate::logger::YELLOW,
            $crate::logger::RESET,
            $crate::logger::GRAY,
            $crate::logger::RESET,
            format!($($arg)*)
        );
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        println!("{}[{}{}ERROR{}{}]{} {}",
            $crate::logger::GRAY,
            $crate::logger::RESET,
            $crate::logger::RED,
            $crate::logger::RESET,
            $crate::logger::GRAY,
            $crate::logger::RESET,
            format!($($arg)*)
        );
    };
}