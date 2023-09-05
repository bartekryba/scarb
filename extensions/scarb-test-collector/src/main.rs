use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use scarb_test_collector::{collect_tests, LinkedLibrary};

#[derive(Parser, Clone, Debug)]
#[command(author, version)]
struct Args {
    #[arg(long)]
    input_path: String,

    #[arg(long)]
    output_path: String,

    #[arg(long)]
    package_name: String,

    #[arg(long)]
    linked_libraries_names: Option<Vec<String>>,

    #[arg(long)]
    linked_libraries_paths: Option<Vec<PathBuf>>,

    #[arg(long)]
    builtins: Option<Vec<String>>,

    #[arg(long)]
    corelib_path: PathBuf,
}

fn main() -> Result<()> {
    println!("Starting work");
    let args: Args = Args::parse();

    let builtins = args.builtins.unwrap_or(Vec::new());

    let linked_libraries = if let (Some(names), Some(paths)) =
        (args.linked_libraries_names, args.linked_libraries_paths)
    {
        assert_eq!(names.len(), paths.len());

        Some(
            names
                .iter()
                .zip(paths)
                .map(|(name, path)| LinkedLibrary {
                    name: name.clone(),
                    path,
                })
                .collect(),
        )
    } else {
        None
    };

    let result = collect_tests(
        &args.input_path,
        Some(&args.output_path),
        &args.package_name,
        linked_libraries,
        &builtins,
        args.corelib_path,
    );

    if let Err(error) = result {
        println!("Error: {error}");
    } else if let Ok(success) = result {
        println!("Success: {:?}", success);
    }

    Ok(())
}
