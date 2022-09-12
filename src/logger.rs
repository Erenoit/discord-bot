use colored::*;

#[inline]
pub fn info(main_str: String, secondary_str: Option<String>) {
    main_print(main_str, LogType::Info);

    if let Some(s) = secondary_str {
        secondary_info(s);
    }
}

#[inline]
pub fn log(main_str: String, secondary_str: Option<String>) {
    main_print(main_str, LogType::Log);

    if let Some(s) = secondary_str {
        secondary_log(s);
    }
}

#[inline]
pub fn warn(main_str: String, secondary_str: Option<String>) {
    main_print(main_str, LogType::Warn);

    if let Some(s) = secondary_str {
        secondary_warn(s);
    }
}

#[inline]
pub fn error(main_str: String, secondary_str: Option<String>) {
    main_print(main_str, LogType::Error);

    if let Some(s) = secondary_str {
        secondary_error(s);
    }
}

#[inline]
pub fn secondary_info(secondary_str: String) {
    secondary_print(secondary_str, LogType::Info);
}

#[inline]
pub fn secondary_log(secondary_str: String) {
    secondary_print(secondary_str, LogType::Log);
}

#[inline]
pub fn secondary_warn(secondary_str: String) {
    secondary_print(secondary_str, LogType::Warn);
}

#[inline]
pub fn secondary_error(secondary_str: String) {
    secondary_print(secondary_str, LogType::Error);
}

#[inline]
fn main_print(main_str: String, log_type: LogType) {
    let chr = match log_type {
        LogType::Info  => "[I]".blue(),
        LogType::Log   => "[L]".green(),
        LogType::Warn  => "[W]".yellow(),
        LogType::Error => "[E]".red(),
    };

    let m_str = match log_type {
        LogType::Info  => main_str.blue(),
        LogType::Log   => main_str.green(),
        LogType::Warn  => main_str.yellow(),
        LogType::Error => main_str.red(),
    };

    println!("{} {}", chr, m_str);
}

#[inline]
fn secondary_print(secondary_str: String, log_type: LogType) {
    let s_str = match log_type {
        LogType::Info  => secondary_str.blue(),
        LogType::Log   => secondary_str.green(),
        LogType::Warn  => secondary_str.yellow(),
        LogType::Error => secondary_str.red(),
    };

    println!("\t{}", s_str);
}

enum LogType {
    Info,
    Log,
    Warn,
    Error,
}

