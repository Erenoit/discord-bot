use std::{env, fs, io::Error, path::Path};

use clap::{CommandFactory, ValueEnum};
use clap_complete::{generate_to, shells};

mod fix_super {
    include!("src/config/cmd_arguments.rs");
}

use fix_super::CMDArguments;

fn main() -> Result<(), Error> {
    let outdir: &Path = Path::new("./completions");
    let pkg_name: &str = env!("CARGO_PKG_NAME");

    if !outdir.exists() {
        fs::create_dir_all(outdir)?;
    }

    let mut cmd = CMDArguments::command();

    for shell in shells::Shell::value_variants() {
        let path = generate_to(*shell, &mut cmd, pkg_name, outdir)?;

        println!("cargo:warning=Completion file is generated: {path:?}");
    }

    Ok(())
}
