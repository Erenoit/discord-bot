#[macro_export]
macro_rules! get_value {
    ($config_file: ident, $ttype: tt, $env_name: literal, $($p: expr)=>+, $default_value: ident) => {
        get_value_common!($config_file, $ttype, $env_name, $($p)=>+, { <$ttype>::from($default_value) })
    };
    ($config_file: ident, $ttype: tt, $env_name: literal, $($p: expr)=>+, $err_message: literal) => {
        get_value_common!($config_file, $ttype, $env_name, $($p)=>+, {
            logger::error($err_message);
            process::exit(1);
        })
    }
}

macro_rules! get_value_common {
    ($config_file: ident, $ttype: tt, $env_name: literal, $($p: expr)=>+, $else: block) => (
        if let Ok(value) = env::var($env_name) {
            if let Ok(val) = value.parse::<$ttype>() { val }
            else {
                logger::error(format!("{} has wrong type", $env_name));
                process::exit(1);
            }
        }
        else if let Some(value) = get_as!($ttype, $config_file.$(get($p)).+) { convert_value!($ttype, value.value()) }
        else { $else }
    )
}

macro_rules! get_as {
    (String, $node: expr) => (
        $node.as_str()
    );
    (bool, $node: expr) => (
        $node.as_bool()
    );
    (u8, $node: expr) => (
        $node.as_integer()
    );
    (PathBuf, $node: expr) => (
        $node.as_str()
    );
}

macro_rules! convert_value {
    (String, $value: expr) => (
        $value.to_string()
    );
    (u8, $value: expr) => (
        if let Some(v) = $value.as_positive() {
            v as u8
        } else {
            logger::error(format!("{} should be positive integer", $value.as_negative().unwrap()));
            process::exit(1);
        }
    );
    (PathBuf, $value: expr) => (
        PathBuf::from($value)
    );
    ($any: tt, $value: expr) => (
        $value
    )
}

