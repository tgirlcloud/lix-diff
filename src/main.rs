use std::{env, path::PathBuf};

use argh::FromArgs;
use color_eyre::Result;
use diff::PackageListDiff;
use nu_ansi_term::{Color, Style};

mod diff;
mod package;
mod parser;
mod versioning;

use self::parser::DiffRoot;

#[derive(FromArgs, PartialEq, Debug)]
/// List the package differences between two `NixOS` generations
struct Args {
    /// the path to the lix bin directory
    #[argh(option, short = 'l')]
    lix_bin: Option<PathBuf>,

    /// the generation we are switching from
    #[argh(positional)]
    before: PathBuf,

    /// the generation we are switching to
    #[argh(positional, default = "PathBuf::from(\"/run/current-system/\")")]
    after: PathBuf,
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();
    let before = args.before;
    let after = args.after;
    let lix_bin = args.lix_bin;

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
