// TODO: remove repeated code
#[macro_export]
macro_rules! get_value {
    ($config_file: ident, bool, $env_name: literal, $($p: expr)=>+, $default_value: ident) => {
        if let Ok(value) = env::var($env_name) { value.to_lowercase() == "true" }
        else if let Some(value) = $config_file.$(get($p)).+.as_bool() { value.value() }
        else { $default_value as bool }
    };
    ($config_file: ident, $type: ty, $env_name: literal, $($p: expr)=>+, $default_value: ident) => {
        get_value_common!($config_file, $type, $env_name, $($p)=>+, { <$type>::from($default_value) })
    };
    ($config_file: ident, $type: ty, $env_name: literal, $($p: expr)=>+, $err_message: literal) => {
        get_value_common!($config_file, $type, $env_name, $($p)=>+, {
            logger::error($err_message);
            process::exit(1);
        })
    }
}

macro_rules! get_value_common {
    ($config_file: ident, $type: ty, $env_name: literal, $($p: expr)=>+, $else: block) => (
        if let Ok(value) = env::var($env_name) { <$type>::from(value) }
        else if let Some(value) = $config_file.$(get($p)).+.as_str() { <$type>::from(value.value()) }
        else { $else }
    )
}

