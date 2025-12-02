use std::path::PathBuf;

use clap::Parser;
use color_eyre::Result;
use diff::PackageListDiff;
use nu_ansi_term::{Color, Style};

mod diff;
mod package;
mod parser;
mod versioning;

use self::parser::DiffRoot;

#[derive(Parser, PartialEq, Debug)]
/// List the package differences between two `NixOS` generations
struct Args {
    /// the path to the lix bin directory
    #[arg(short, long)]
    lix_bin: Option<PathBuf>,

    /// the generation we are switching from
    before: PathBuf,

    /// the generation we are switching to
    #[arg(default_value = "/run/current-system/")]
    after: PathBuf,

    /// sort by size difference
    #[arg(short, long)]
    size: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let before = args.before;
    let after = args.after;
    let lix_bin = args.lix_bin;

    let mut lix_exe = None;
    if let Some(lix_bin) = lix_bin {
        lix_exe = if lix_bin.is_dir() {
            Some(lix_bin.join("nix"))
        } else {
            Some(lix_bin)
        }
    }

    if !before.exists() {
        eprintln!("Before generation does not exist: {}", before.display());
        std::process::exit(1);
    }

    if !after.exists() {
        eprintln!("After generation does not exist: {}", after.display());
        std::process::exit(1);
    }

    let packages_diff = DiffRoot::new(lix_exe, &before, &after)?;
    let mut packages: PackageListDiff = PackageListDiff::new();
    packages.by_size = args.size;
    packages.from_diff_root(packages_diff);

    let arrow_style = Style::new().bold().fg(Color::LightGray);

    let before_text = format!("<<< {}", before.display());
    let after_text = format!(">>> {}", after.display());

    println!("{}", arrow_style.paint(before_text));
    println!("{}\n", arrow_style.paint(after_text));

    println!("{packages}");

    Ok(())
}
