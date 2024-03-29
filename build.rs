//! Creates shell autocompletions

use std::io::Error;
#[cfg(feature = "cmd")]
use std::{env, fs, path::Path};

#[cfg(feature = "cmd")]
use clap::{CommandFactory, ValueEnum};
#[cfg(feature = "cmd")]
use clap_complete::{generate_to, shells};

#[cfg(feature = "cmd")]
mod fix_super {
    //! import `cmd_arguments` to be able to generate autocommpletions from
    include!("src/config/cmd_arguments.rs");
}

#[cfg(feature = "cmd")]
use fix_super::CMDArguments;

fn main() -> Result<(), Error> {
    #[cfg(feature = "cmd")]
    {
        let outdir = Path::new("./completions");
        let pkg_name = env!("CARGO_PKG_NAME");

        if !outdir.exists() {
            fs::create_dir_all(outdir)?;
        }

        let mut cmd = CMDArguments::command();

        for shell in shells::Shell::value_variants() {
            let path = generate_to(*shell, &mut cmd, pkg_name, outdir)?;

            println!("cargo:warning=Completion file is generated: {path:?}");
        }
    }

    Ok(())
}
