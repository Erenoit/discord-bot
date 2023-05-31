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
