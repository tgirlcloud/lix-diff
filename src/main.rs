use std::{env, path::PathBuf};

use clap::Parser;
use color_eyre::Result;
use diff::PackageListDiff;
use nu_ansi_term::{Color, Style};

mod diff;
mod package;
mod parser;
mod versioning;

use self::parser::DiffRoot;

#[derive(Debug, Parser)]
#[command(multicall = true)]
#[command(name = "lemon-sorbet")]
#[command(bin_name = "lemon-sorbet")]
#[clap(about, version)]
enum Cli {
    LixDiff(LixDiffArgs),
    /// The multicall binary command so we can call the generated binary without creating a symlink
    /// first
    #[command(hide = true)]
    LemonSorbet {
        args: Vec<String>,
    },
}

#[derive(clap::Args, PartialEq, Debug)]
/// List the package differences between two `NixOS` generations
struct LixDiffArgs {
    /// the path to the lix bin directory
    #[arg(short, long)]
    lix_bin: Option<PathBuf>,

    /// the generation we are switching from
    before: PathBuf,

    /// the generation we are switching to
    #[arg(default_value = "/run/current-system/")]
    after: PathBuf,
}

impl LixDiffArgs {
    fn process(self) -> Result<()> {
        let before = self.before;
        let after = self.after;
        let lix_bin = self.lix_bin;

        if let Some(lix_bin) = lix_bin {
            let current_path = env::var_os("PATH").unwrap();
            let current_path = env::split_paths(&current_path);
            let new_path = std::iter::once(lix_bin).chain(current_path);
            let new_path = env::join_paths(new_path).unwrap();
            unsafe {
                env::set_var("PATH", &new_path);
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

        let packages: PackageListDiff = DiffRoot::new(&before, &after)?.into();

        let arrow_style = Style::new().bold().fg(Color::LightGray);

        let before_text = format!("<<< {}", before.display());
        let after_text = format!(">>> {}", after.display());

        println!("{}", arrow_style.paint(before_text));
        println!("{}\n", arrow_style.paint(after_text));

        println!("{packages}");

        Ok(())
    }
}

impl Cli {
    fn process(self) -> Result<()> {
        match self {
            Cli::LemonSorbet { args } => Cli::parse_from(args).process(),
            Cli::LixDiff(args) => args.process(),
        }
    }
}

fn main() -> Result<()> {
    Cli::parse().process()
}
