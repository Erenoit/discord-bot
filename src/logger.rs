//! This module contains a simple logger macro.

/// Main macro for logging.
///
/// The output is colorful to make log type more distinguishable
///
/// All the logs written in `stderr`. If you want to write to `stdout` or print
/// some other things please use good old `println!()`.
///
/// Usage:
/// ```rust
/// log!(info, "This is an info log with JUST main message");
/// log!(warn, ; "This is a warning log with JUST secondary message");
/// log!(error, "This is an error log with main message and secondary message"; "This is a secondary message");
/// log!(info, "This is an info log with main message and multiple secondary messages";
///     "This is a secondary message";
///     "This is another secondary message");
/// ```
///
/// You can alsu use format!() like expressions in main and secondary messages.
/// ```rust
/// log!(info, "This is an info log with main message and {multiple} secondary messages", multiple = 2;
///    "This is a secondary message";
///    "This is another secondary message with a number: {}", 42);
/// ```
#[macro_export]
macro_rules! log {
    (info,  ; $($($secondary_message: tt),*);*) => { log_common!(blue,    "", $($($secondary_message),*);*); };
    (warn,  ; $($($secondary_message: tt),*);*) => { log_common!(cyan,    "", $($($secondary_message),*);*); };
    (error, ; $($($secondary_message: tt),*);*) => { log_common!(magenta, "", $($($secondary_message),*);*); };
    (info,  $($main_message: tt),*) => { log_common!(green,  "[I]", $($main_message),*); };
    (warn,  $($main_message: tt),*) => { log_common!(yellow, "[W]", $($main_message),*); };
    (error, $($main_message: tt),*) => { log_common!(red,    "[E]", $($main_message),*); };
    (info,  $($main_message: tt),*; $($($secondary_message: tt),*);*) => { log_common!(green,  blue,    "[I]", $($main_message),*; $($($secondary_message),*);*); };
    (warn,  $($main_message: tt),*; $($($secondary_message: tt),*);*) => { log_common!(yellow, cyan,    "[W]", $($main_message),*; $($($secondary_message),*);*); };
    (error, $($main_message: tt),*; $($($secondary_message: tt),*);*) => { log_common!(red,    magenta, "[E]", $($main_message),*; $($($secondary_message),*);*); };
}

/// This is an inner function for `log!()` macro. Do not use!
macro_rules! log_common {
    ($c1: ident, $c2: ident, $symbol: literal, $($main_message: tt),*; $($($secondary_message: tt),*);*) => {
        log_common_wrapper!({
            print_log!(main, $c1, $symbol, $($main_message),*);
            print_log!(secondary, $c2, $($($secondary_message),*);*);
        })
    };
    ($color: ident, "", $($($message: tt),*);*) => {
        log_common_wrapper!({
            print_log!(secondary, $color, $($($message),*);*);
        })
    };
    ($color: ident, $symbol: literal, $($message: tt),*) => {
        log_common_wrapper!({
            print_log!(main, $color, $symbol, $($message),*);
        })
    };
}

/// This is an inner function for `log!()` macro. Do not use!
macro_rules! log_common_wrapper {
    ($code: block) => {
        {
            use std::io::{stdout, Write};
            use colored::Colorize;

            $code

            stdout().flush().unwrap();
        }
    }
}

// TODO: add time and some other extra info
/// This is an inner function for `log!()` macro. Do not use!
macro_rules! print_log {
    (main, $color: ident, $symbol: literal, $($message: tt),*) => {
        eprint!("{} {}\n", $symbol.$color(), format!($($message),*).$color());
    };
    (secondary, $color: ident, $($($message: tt),*);*) => {
        $(
            format!($($message),*).split('\n').for_each(|line| {
                eprint!("\t{}\n", line.$color());
            });
        );*
    };
}
