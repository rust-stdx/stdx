use std::{
    collections::HashMap,
    env, io,
    path::{Path, PathBuf},
    process,
    sync::{Arc, Mutex},
    thread,
};

use crypto::{Hasher, blake3::Blake3};
use tokio::{fs::File, io::AsyncReadExt, sync::Semaphore, task};
use walkdir::WalkDir;

/// Scans one or more directories for duplicate files and prints groups
/// sorted by total disk space used (size × occurrence count).
///
/// Each group lists the human-readable total size, the BLAKE3 hash, and
/// every path sharing that hash.
///
/// # Errors
///
/// Exits with a non-zero status if fewer than one directory argument is
/// provided. Individual file-hashing errors are printed to stderr and
/// do not halt the scan.
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: duplicates <folder1> [folder2 ...]");
        process::exit(1);
    }

    let results: Arc<Mutex<HashMap<[u8; 32], (u64, Vec<PathBuf>)>>> = Arc::new(Mutex::new(HashMap::new()));
    let semaphore = Arc::new(Semaphore::new(thread::available_parallelism().unwrap().get()));
    let mut handles = Vec::with_capacity(args.len() - 1);

    for arg in &args[1..] {
        let dir = PathBuf::from(arg);
        let results = Arc::clone(&results);
        let semaphore = Arc::clone(&semaphore);
        let handle = task::spawn(async move {
            if let Err(e) = process_directory(&dir, &results, &semaphore).await {
                eprintln!("error processing '{}': {e}", dir.display());
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    let map = results.lock().unwrap();
    let mut groups: Vec<(u64, &[u8; 32])> = map
        .iter()
        .filter(|(_, (_, paths))| paths.len() > 1)
        .map(|(hash, (size, paths))| (size * paths.len() as u64, hash))
        .collect();
    groups.sort_by(|a, b| b.0.cmp(&a.0));

    for (total, hash) in groups {
        let (_, paths) = &map[hash];
        println!("{}: {}:", format_size(total), hex::encode(hash));
        let mut sorted = paths.clone();
        sorted.sort();
        for p in &sorted {
            println!("    {}", p.display());
        }
    }
}

async fn process_directory(
    dir: &Path,
    results: &Arc<Mutex<HashMap<[u8; 32], (u64, Vec<PathBuf>)>>>,
    semaphore: &Arc<Semaphore>,
) -> Result<(), io::Error> {
    let mut handles = vec![];

    for entry in WalkDir::new(dir).follow_links(false) {
        let entry = entry.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        if entry.file_type().is_file() && !entry.file_type().is_symlink() {
            let path = entry.into_path();
            let results = Arc::clone(results);
            let semaphore = Arc::clone(semaphore);
            let handle = task::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                match hash_file(&path).await {
                    Ok((hash, size)) => {
                        let mut map = results.lock().unwrap();
                        map.entry(hash).or_insert_with(|| (size, Vec::new())).1.push(path);
                    }
                    Err(e) => {
                        eprintln!("error hashing '{}': {e}", path.display());
                    }
                }
            });
            handles.push(handle);
        }
    }

    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}

async fn hash_file(path: &Path) -> Result<([u8; 32], u64), io::Error> {
    let mut file = File::open(path).await?;
    let size = file.metadata().await?.len();
    let mut hasher = Blake3::new();
    let mut buf = vec![0u8; 1024 * 1024];

    loop {
        let n = file.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    let hash = hasher.sum();
    Ok((hash.as_ref().try_into().unwrap(), size))
}

fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut val = bytes as f64;
    let mut idx = 0;
    while val >= 1024.0 && idx < UNITS.len() - 1 {
        val /= 1024.0;
        idx += 1;
    }
    format!("{:.0}{}", val, UNITS[idx])
}
