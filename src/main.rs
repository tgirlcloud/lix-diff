use std::path::PathBuf;

use clap::Parser;
use color_eyre::Result;
use nu_ansi_term::{Color, Style};
use shellexpand::path::tilde;
use terminal_light::luma;

use lix_diff::{DiffRoot, PackageListDiff, color};

#[allow(clippy::struct_excessive_bools)]
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

    /// disable colored output (also respects `NO_COLOR` env var and CI environments)
    #[arg(long)]
    no_color: bool,

    /// disable the header showing the generation paths
    #[arg(long)]
    no_header: bool,

    /// add a return code indicating whether there are changes or no (1 if there are no changes, 0
    /// otherwise)
    #[arg(short, long = "return")]
    return_code: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    color::init(args.no_color);

    let before = tilde(&args.before);
    let after = tilde(&args.after);
    let lix_bin = args.lix_bin.as_deref().map(tilde);

    let mut lix_exe = None;
    if let Some(lix_bin) = lix_bin {
        lix_exe = if lix_bin.is_dir() {
            Some(lix_bin.join("nix").into())
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

    let packages_diff = DiffRoot::new(lix_exe.as_deref(), &before, &after)?;
    let mut packages: PackageListDiff = PackageListDiff::new();
    packages.by_size = args.size;
    packages.from_diff_root(packages_diff);

    let before_text = format!("<<< {}", before.display());
    let after_text = format!(">>> {}", after.display());

    if !args.no_header {
        if color::color_enabled() {
            let text_color = if luma().is_ok_and(|luma| luma > 0.6) {
                Color::DarkGray
            } else {
                Color::LightGray
            };
            let arrow_style = Style::new().bold().fg(text_color);
            println!("{}", arrow_style.paint(&before_text));
            println!("{}\n", arrow_style.paint(&after_text));
        } else {
            println!("{before_text}");
            println!("{after_text}\n");
        }
    }

    println!("{packages}");

    if args.return_code {
        std::process::exit(packages.is_empty().into());
    }

    Ok(())
}
