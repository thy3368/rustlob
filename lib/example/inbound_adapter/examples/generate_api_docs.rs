use std::path::PathBuf;

fn main() {
    if let Err(error) = run() {
        eprintln!("generate_api_docs failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../");
    let repo_root = repo_root.canonicalize()?;
    let written = example_inbound_adapter::write_generated_api_docs(&repo_root)?;

    for path in written {
        println!("{}", path.display());
    }

    Ok(())
}
