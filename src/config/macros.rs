//! This module contains macros for config module

/// Gets config value from config sources for given parameters
///
/// # Parameters
/// - `config_file`: config file read by taplo
/// - `ttype`: type of the config value
/// - `env_name`: environmental variable name to get the config from
/// - `toml config file path`: path to the variable inside config file,
///   seperated with arrows.
/// - for the last part you have two options:
///     - `default_value`: default value if value is not given either
///       environmental variable or config file
///     - `err_message`: error message will be displayed in [`anyhow::Result`]
///       if not value is given either in environment variable or config file
///
/// For examples you can check [`Config::generate()`]
///
/// [`Config::generate()`]: crate::config::Config::generate()
#[macro_export]
macro_rules! get_value {
    ($config_file: ident, $ttype: tt, $env_name: literal, $($p: expr)=>+, $default_value: ident) => {
        get_value_common!($config_file, $ttype, $env_name, $($p)=>+, { anyhow::Ok(<$ttype>::from($default_value)) })
    };
    ($config_file: ident, $ttype: tt, $env_name: literal, $($p: expr)=>+, $err_message: literal) => {
        get_value_common!($config_file, $ttype, $env_name, $($p)=>+, {
            log!(error, $err_message);
            Err(anyhow::anyhow!("No value is given"))
        })
    }
}

/// This is an inner function for [`get_value!()`] macro. Do not use!
macro_rules! get_value_common {
    ($config_file: ident, $ttype: tt, $env_name: literal, $($p: expr)=>+, $else: block) => {
        {
            use std::env;

            #[cfg(feature = "config_file")]
            if let Ok(value) = env::var($env_name) {
                anyhow::Ok(value.parse::<$ttype>()?)
            }
            else if let Some(value) = get_as!($ttype, $config_file.$(get($p)).+) { convert_value!($ttype, value.value()) }
            else { $else }

            #[cfg(not(feature = "config_file"))]
            let _ = $config_file;

            #[cfg(not(feature = "config_file"))]
            if let Ok(value) = env::var($env_name) {
                anyhow::Ok(value.parse::<$ttype>()?)
            }
            else { $else }

        }
    }
}

/// This is an inner function for [`get_value!()`] macro. Do not use!
#[cfg(feature = "config_file")]
macro_rules! get_as {
    (String, $node:expr) => {
        $node.as_str()
    };
    (bool, $node:expr) => {
        $node.as_bool()
    };
    (u8, $node:expr) => {
        $node.as_integer()
    };
    (u32, $node:expr) => {
        $node.as_integer()
    };
    (u64, $node:expr) => {
        $node.as_integer()
    };
    (PathBuf, $node:expr) => {
        $node.as_str()
    };
}

/// This is an inner function for [`get_value!()`] macro. Do not use!
#[cfg(feature = "config_file")]
macro_rules! convert_value {
    (String, $value:expr) => {
        Ok($value.to_owned())
    };
    (u8, $value:expr) => {
        if let Some(v) = $value.as_positive() {
            Ok(v as u8)
        } else {
            log!(
                error,
                "{} should be positive integer",
                ($value.as_negative().unwrap())
            );
            Err(anyhow::anyhow!("u8 cannot be negative"))
        }
    };
    (u32, $value:expr) => {
        if let Some(v) = $value.as_positive() {
            Ok(v as u32)
        } else {
            log!(
                error,
                "{} should be positive integer",
                ($value.as_negative().unwrap())
            );
            Err(anyhow::anyhow!("u32 cannot be negative"))
        }
    };
    (u64, $value:expr) => {
        if let Some(v) = $value.as_positive() {
            Ok(v as u64)
        } else {
            log!(
                error,
                "{} should be positive integer",
                ($value.as_negative().unwrap())
            );
            Err(anyhow::anyhow!("u64 cannot be negative"))
        }
    };
    (PathBuf, $value:expr) => {
        Ok(PathBuf::from($value))
    };
    ($any:tt, $value:expr) => {
        anyhow::Ok($value)
    };
}
