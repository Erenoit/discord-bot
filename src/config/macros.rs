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

macro_rules! get_value_common {
    ($config_file: ident, $ttype: tt, $env_name: literal, $($p: expr)=>+, $else: block) => (
        if let Ok(value) = env::var($env_name) {
            anyhow::Ok(value.parse::<$ttype>()?)
        }
        else if let Some(value) = get_as!($ttype, $config_file.$(get($p)).+) { convert_value!($ttype, value.value()) }
        else { $else }
    )
}

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
