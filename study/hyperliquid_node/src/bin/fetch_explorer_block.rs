use std::error::Error;
use std::ffi::OsStr;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fmt, process};

use hyperliquid_node::explorer_block::{
    OFFICIAL_EXPLORER_URL, fetch_explorer_bytes, parse_explorer_block,
};

#[derive(Debug)]
struct UsageError;

impl fmt::Display for UsageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("usage: fetch_explorer_block <HEIGHT> [OUTPUT_JSON]")
    }
}

impl Error for UsageError {}

fn main() {
    if let Err(error) = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .and_then(|runtime| runtime.block_on(run()).map_err(std::io::Error::other))
    {
        eprintln!("{error}");
        process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut args = env::args_os().skip(1);
    let height = args.next().ok_or(UsageError)?;
    let height = height.to_str().ok_or(UsageError)?.parse::<u64>()?;
    let output = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(format!("explorer_block_{height}.json")));
    if args.next().is_some() {
        return Err(UsageError.into());
    }

    let endpoint =
        env::var("HYPERLIQUID_EXPLORER_URL").unwrap_or_else(|_| OFFICIAL_EXPLORER_URL.to_owned());
    let raw = fetch_explorer_bytes(&endpoint, height).await?;
    if let Err(error) = parse_explorer_block(raw.clone(), height) {
        let evidence = unparsed_path(&output);
        atomic_write(&evidence, &raw)?;
        return Err(format!("{error}; raw response saved to {}", evidence.display()).into());
    }
    atomic_write(&output, &raw)?;
    println!("{}", output.display());
    Ok(())
}

fn unparsed_path(output: &Path) -> PathBuf {
    let mut name = output
        .file_name()
        .unwrap_or_else(|| OsStr::new("explorer_block"))
        .to_os_string();
    name.push(".unparsed.json");
    output.with_file_name(name)
}

fn atomic_write(output: &Path, bytes: &[u8]) -> Result<(), Box<dyn Error + Send + Sync>> {
    let parent = output.parent().unwrap_or_else(|| Path::new("."));
    let file_name = output.file_name().ok_or("output path has no file name")?;
    let mut temporary_name = file_name.to_os_string();
    temporary_name.push(format!(".{}.tmp", process::id()));
    let temporary = parent.join(temporary_name);

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)?;
    if let Err(error) = file.write_all(bytes).and_then(|()| file.sync_all()) {
        let _ = fs::remove_file(&temporary);
        return Err(error.into());
    }
    drop(file);
    if output.exists() {
        let _ = fs::remove_file(&temporary);
        return Err(format!("refusing to overwrite existing file: {}", output.display()).into());
    }
    if let Err(error) = fs::rename(&temporary, output) {
        let _ = fs::remove_file(&temporary);
        return Err(error.into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives_unparsed_evidence_name() {
        assert_eq!(
            unparsed_path(Path::new("block.json")),
            Path::new("block.json.unparsed.json")
        );
    }
}
