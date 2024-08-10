use std::time::Instant;
use std::{path::PathBuf, process::Command};

use clap::Parser;
use wax::Glob;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to look for scripts to run
    #[arg(short, long)]
    path: PathBuf,
}
use color_eyre::eyre::{OptionExt, Result};

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    if args.path.is_file() {
        run_cmd(args.path)?;
        return Ok(());
    }
    let glob = Glob::new("*/{scripts,src/bin}/*.{rs,js,py}")?;
    for entry in glob.walk(args.path) {
        let entry = entry.unwrap();
        let entry = entry.into_path();
        run_cmd(entry)?;
    }
    Ok(())
}

fn run_cmd(entry: PathBuf) -> Result<()> {
    match entry
        .extension()
        .ok_or_eyre("No extension")?
        .to_str()
        .ok_or_eyre("String conversion failed")?
    {
        "rs" => {
            let stem = entry
                .file_stem()
                .ok_or_eyre("No stem")?
                .to_str()
                .ok_or_eyre("String conversion failed")?;
            let cwd = entry
                .parent()
                .ok_or_eyre("Bad path")?
                .parent()
                .ok_or_eyre("Bad path")?
                .parent()
                .ok_or_eyre("Bad path")?;
            Command::new("cargo")
                .args(["build", "--release", "--bin", stem])
                .current_dir(cwd)
                .spawn()?
                .wait()?;
            let now = Instant::now();
            Command::new("cargo")
                .args(["run", "--release", "--bin", stem])
                .current_dir(cwd)
                .spawn()?
                .wait()?;
            let elapsed = now.elapsed();
            println!("Elapsed: {:.2?} for [{}] in {{Rust}}", elapsed, stem,);
        }
        "py" => {
            let stem = entry
                .file_stem()
                .ok_or_eyre("No stem")?
                .to_str()
                .ok_or_eyre("String conversion failed")?;
            let cwd = entry
                .parent()
                .ok_or_eyre("Bad path")?
                .parent()
                .ok_or_eyre("Bad path")?;
            let mut command = Command::new("python");
            let command = command.args([entry.clone()]);
            if let Some(venv) = Glob::new("{.env,venv,.venv}")?.walk(cwd).last() {
                command.env("PATH", venv?.into_path().join("scripts"));
            }
            let now = Instant::now();
            command.spawn()?.wait()?;
            let elapsed = now.elapsed();
            println!("Elapsed: {:.2?} for [{}] in {{Python}}", elapsed, stem);
        }
        _ => {}
    }
    Ok(())
}
