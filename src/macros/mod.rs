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
#[macro_export]
macro_rules! cmd {
    ($command:expr) => {{
        use std::process::{Command, Stdio};

        let mut command = Command::new(&$command[0]);
        for arg in &$command[1..] {
            command.arg(arg);
        }

        let cmd = command
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn();

        match cmd {
            Ok(mut child) => {
                let status = child.wait();
                if let Err(err) = status {
                    println!("Error waiting for process: {}", err.red().bold());
                }
            }
            Err(err) => {
                println!("Error spawning process: {}", err.red().bold());
            }
        }
    }};
}
