#[macro_export]
macro_rules! warning {
    ($message:expr) => {
        println!(
            "{} {}",
            "[  ]".yellow().bold(),
            $message
        );
    };
    ($message:expr, $($arg:expr),*) => {
        println!(
            "{} {}",
            "[  ]".yellow().bold(),
            format!($message, $($arg),*)
        );
    };
}
#[macro_export]
macro_rules! error {
    ($message:expr) => {
        println!(
            "{} {}",
            "[  ]".red().bold(),
            $message
        );
    };
    ($message:expr, $($arg:expr),*) => {
        println!(
            "{} {}",
            "[  ]".red().bold(),
            format!($message, $($arg),*)
        );
    };
}
#[macro_export]
macro_rules! success {
    ($message:expr) => {
        println!(
            "{} {}",
            "[  ]".green().bold(),
            $message
        );
    };
    ($message:expr, $($arg:expr),*) => {
        println!(
            "{} {}",
            "[  ]".green().bold(),
            format!($message, $($arg),*)
        );
    };
}
