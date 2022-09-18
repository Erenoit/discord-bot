use std::fmt::Display;
use colored::Colorize;

#[inline(always)]
pub fn info<S: Display>(main_str: S) {
    main_print(main_str, LogType::Info);
}

#[inline(always)]
pub fn warn<S: Display>(main_str: S) {
    main_print(main_str, LogType::Warn);
}

#[inline(always)]
pub fn error<S: Display>(main_str: S) {
    main_print(main_str, LogType::Error);
}

#[inline(always)]
pub fn secondary_info<S: Display>(secondary_str: S) {
    secondary_print(secondary_str, LogType::Info);
}

#[inline(always)]
pub fn secondary_warn<S: Display>(secondary_str: S) {
    secondary_print(secondary_str, LogType::Warn);
}

#[inline(always)]
pub fn secondary_error<S: Display>(secondary_str: S) {
    secondary_print(secondary_str, LogType::Error);
}

#[inline(always)]
fn main_print<S: Display>(main_str: S, log_type: LogType) {
    let chr = match log_type {
        LogType::Info  => "[I]".green(),
        LogType::Warn  => "[W]".yellow(),
        LogType::Error => "[E]".red(),
    };

    let m_str = match log_type {
        LogType::Info  => format!("{}", main_str).green(),
        LogType::Warn  => format!("{}", main_str).yellow(),
        LogType::Error => format!("{}", main_str).red(),
    };

    println!("{} {}", chr, m_str);
}

#[inline(always)]
fn secondary_print<S: Display>(secondary_str: S, log_type: LogType) {
    let s_str = match log_type {
        LogType::Info  => format!("{}", secondary_str).blue(),
        LogType::Warn  => format!("{}", secondary_str).cyan(),
        LogType::Error => format!("{}", secondary_str).magenta(),
    };

    println!("\t{}", s_str);
}

enum LogType {
    Info,
    Warn,
    Error,
}

