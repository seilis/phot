use anyhow::{Context, Result};
use clap::{Arg, App, SubCommand};
use std::path::PathBuf;

use phot::library::Library;

fn main() -> Result<()> {

    let args = App::new("phot")
                .version("0.1")
                .author("Aaron Seilis <aaron.seilis@seilis.ca>")
                .about("Photo library management tool")
                .subcommand(
                    SubCommand::with_name("new")
                        .about("make a new library")
                )
                .subcommand(
                    SubCommand::with_name("add")
                        .about("add an image or group of images to the library")
                        .arg(
                            Arg::with_name("input")
                                .help("the file or directory to add to the library")
                                .index(1)
                                .required(true),
                            )
                )
                .get_matches();

    match args.subcommand() {
        ("new", Some(new_args)) => {
            let lib = Library::new();
            lib.create().with_context(|| format!("Failed to create a new library"))?;
            println!("Created a library at {}", lib.get_path().to_str().expect("no library path"));
        }
        ("add", Some(add_args)) => {
            let mut lib = Library::new();
            let path = add_args.value_of("input").unwrap();
            let path: PathBuf = path.into();
            lib.add_file(path.as_path()).with_context(|| format!("Failed to add path {:?}", path))?;
        }
        _ => {
            println!("no subcommand");
        }
    };

    Ok(())
}
