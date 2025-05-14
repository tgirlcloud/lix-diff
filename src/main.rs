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
    before: String,

    /// the generation we are switching to
    #[argh(positional)]
    after: String,
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();
    let before = args.before;
    let after = args.after;
    let lix_bin = args.lix_bin;

    if let Some(lix_bin) = lix_bin {
        let current_path = env::var("PATH").unwrap();
        let new_path = format!("{lix_bin}:{current_path}");
        unsafe {
            env::set_var("PATH", &new_path);
        }
    }

    // Check if the generations are valid
    if before.is_empty() || after.is_empty() {
        eprintln!("Both generations must be specified.");
        std::process::exit(1);
    }

    let packages: PackageListDiff = DiffRoot::new(&before, &after)?.into();

    let arrow_style = Style::new().bold().fg(Color::LightGray);

    let before_text = format!("<<< {before}");
    let after_text = format!(">>> {after}");

    println!("{}", arrow_style.paint(before_text));
    println!("{}\n", arrow_style.paint(after_text));

    println!("{packages}");

    Ok(())
}
