use std::{env, fs, io::Error, path::Path};

use clap::CommandFactory;
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

    // TODO: find a way to write this without copy-paste
    let path = generate_to(
        shells::Bash,
        &mut cmd, // We need to specify what generator to use
        pkg_name, // We need to specify the bin name manually
        outdir,   // We need to specify where to write to
    )?;

    println!(
        "cargo:warning=completion file is generated: {:?}",
        path
    );

    let path = generate_to(
        shells::Zsh,
        &mut cmd, // We need to specify what generator to use
        pkg_name, // We need to specify the bin name manually
        outdir,   // We need to specify where to write to
    )?;

    println!(
        "cargo:warning=completion file is generated: {:?}",
        path
    );

    let path = generate_to(
        shells::Fish,
        &mut cmd, // We need to specify what generator to use
        pkg_name, // We need to specify the bin name manually
        outdir,   // We need to specify where to write to
    )?;

    println!(
        "cargo:warning=completion file is generated: {:?}",
        path
    );

    let path = generate_to(
        shells::PowerShell,
        &mut cmd, // We need to specify what generator to use
        pkg_name, // We need to specify the bin name manually
        outdir,   // We need to specify where to write to
    )?;

    println!(
        "cargo:warning=completion file is generated: {:?}",
        path
    );

    Ok(())
}
