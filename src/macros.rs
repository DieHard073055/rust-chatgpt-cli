#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        if std::env::var("ENABLE_DEBUG_PRINT").ok() == Some(String::from("1")) {
            println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => {
        if std::env::var("ENABLE_DEBUG_PRINT").ok() == Some(String::from("1")) {
            print!($($arg)*);
        }
    };
}
