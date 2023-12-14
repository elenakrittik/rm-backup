use std::path::PathBuf;

use dirs::cache_dir;
use merkle_hash::camino::Utf8Path;
use merkle_hash::{Algorithm, MerkleTree};
use xxhash_rust::xxh3::Xxh3;

mod cli;
// mod config;

fn main() -> anyhow::Result<()> {
    // let conf = config::config()?;
    let opts = cli::options()?;

    if opts.print_last_log {
        println!(
            "{}",
            last_log_path()?
                .map(|p| p.display().to_string())
                .unwrap_or("No logs yet.".to_string())
        );
        return Ok(());
    }

    let log_path = cache_folder()?.join(format!("log-{}.txt", current_time()?));

    if !log_path.exists() {
        std::fs::write(&log_path, "")?;
    }

    simple_logging::log_to_file(&log_path, log::LevelFilter::Debug)?;

    for path in opts.paths {
        if !path.exists() {
            log::info!(
                "Path {} does not exist or is unreadable, skipping.",
                abs(&path)?.display(),
            );
            continue;
        }

        if path.is_dir() {
            if !opts.recursive {
                // This is incorrect. `rm` will handle the error,
                // and we will just skip the backup.
                continue;
            }

            backup_dir(path)?;
        } else if path.is_file() {
            backup_file(path)?;
        }
    }

    Ok(())
}

fn backup_dir(path: PathBuf) -> anyhow::Result<()> {
    let cache_dir = cache_folder()?;
    let hash = xx3(hash_dir(&path)?);
    let target = cache_dir.join(&hash);

    if !target.try_exists().is_ok_and(|x| x) {
        let errors = copy_dir::copy_dir(&path, target)?;

        if errors.len() > 0 {
            anyhow::bail!(
                "There were errors backing up {}: {:?}",
                abs(&path)?.display(),
                &errors
            );
        }

        log::info!("Backed up {} as {}", abs(&path)?.display(), &hash);
    } else {
        log::info!("Ignoring {} (already backed up)", abs(&path)?.display());
    }

    Ok(())
}

fn hash_dir(path: &PathBuf) -> anyhow::Result<Vec<u8>> {
    let utf8path = match Utf8Path::from_path(path.as_path()) {
        Some(pth) => pth,
        None => anyhow::bail!("Non-UTF-8 paths are not supported yet."),
    };

    let tree = MerkleTree::builder(utf8path)
        .algorithm(Algorithm::Sha512)
        .hash_names(true)
        .build()?;

    Ok(tree.root.item.hash)
}

fn backup_file(path: PathBuf) -> anyhow::Result<()> {
    let cache_dir = cache_folder()?;
    let content = std::fs::read(&path)?;
    let hash = xx3([path.as_os_str().as_encoded_bytes(), content.as_slice()].concat());
    let target = cache_dir.join(&hash);

    if !target.try_exists().is_ok_and(|x| x) {
        std::fs::write(target, content)?;

        log::info!("Backed up {} as {}", abs(&path)?.display(), &hash);
    } else {
        log::info!("Ignoring {} (already backed up)", abs(&path)?.display());
    }

    Ok(())
}

fn cache_folder() -> anyhow::Result<PathBuf> {
    let path = match cache_dir() {
        Some(pth) => pth.join("rm-backup"),
        None => anyhow::bail!("Unable to find cache directory."),
    };

    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }

    Ok(path)
}

fn xx3(bytes: Vec<u8>) -> String {
    let mut hasher = Xxh3::new();
    hasher.update(bytes.as_slice());
    hasher.digest128().to_string()
}

fn current_time() -> anyhow::Result<String> {
    let now = std::time::SystemTime::now();
    let since_epoch = now.duration_since(std::time::SystemTime::UNIX_EPOCH)?;

    Ok(since_epoch.as_secs().to_string())
}

fn abs(path: &PathBuf) -> anyhow::Result<PathBuf> {
    Ok(std::fs::canonicalize(path)?)
}

fn last_log_path() -> anyhow::Result<Option<PathBuf>> {
    let cache_path = cache_folder()?;

    let mut paths = cache_path
        .read_dir()?
        .into_iter()
        .filter(|v| v.is_ok())
        .map(|v| v.unwrap())
        .filter(|p| {
            p.file_name()
                .to_str()
                .is_some_and(|n| n.starts_with("log-"))
        })
        .map(|v| abs(&v.path()))
        .filter(|v| v.is_ok())
        .map(|v| v.unwrap())
        .collect::<Vec<_>>();

    paths.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    Ok(paths.last().cloned())
}
