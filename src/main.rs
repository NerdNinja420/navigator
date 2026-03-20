use std::fs;
use std::path::{Path, PathBuf};
use std::process;

use loggingg::{err, rcmd};

fn resolve(base: &Path, args: &[String]) -> Result<PathBuf, String> {
    let mut current = base.to_path_buf();

    for arg in args {
        let mut all_matches: Vec<PathBuf> = fs::read_dir(&current)
            .map_err(|e| format!("Cannot read '{}': {}", current.display(), e))?
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
            .filter(|e| e.file_name().to_string_lossy().starts_with(arg.as_str()))
            .map(|e| e.path())
            .collect();

        all_matches.sort();

        // Exact match wins immediately, no ambiguity check
        let exact = all_matches
            .iter()
            .find(|p| p.file_name().unwrap().to_string_lossy() == arg.as_str())
            .cloned();

        if let Some(exact_path) = exact {
            current = exact_path;
            continue;
        }

        match all_matches.len() {
            0 => return Err(format!("No dir matching '{}' in '{}'", arg, current.display())),
            1 => current = all_matches.remove(0),
            _ => {
                let names: Vec<String> = all_matches
                    .iter()
                    .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
                    .collect();
                return Err(format!("Ambiguous '{}':  {}", arg, names.join("  |  ")));
            }
        }
    }

    Ok(current)
}

fn list_completions(base: &Path, args: &[String]) {
    let (to_resolve, prefix) = match args.split_last() {
        Some((last, rest)) => (rest, last.as_str()),
        None => (&args[..0], ""),
    };

    let dir = match resolve(base, to_resolve) {
        Ok(d) => d,
        Err(_) => return,
    };

    let mut names: Vec<String> = fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| n.starts_with(prefix))
        .collect();

    names.sort();
    names.iter().for_each(|n| println!("{}", n));
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let (complete, rest) = match args.first().map(|s| s.as_str()) {
        Some("--complete") => (true, &args[1..]),
        _ => (false, &args[..]),
    };

    if rest.is_empty() {
        err("usage: nav [--complete] <base_dir> [args...]");
        process::exit(1);
    }

    let base = PathBuf::from(&rest[0]);
    let nav_args = rest[1..].to_vec();

    if !base.exists() {
        err(&format!("Base dir does not exist: {}", base.display()));
        process::exit(1);
    }

    if complete {
        list_completions(&base, &nav_args);
    } else {
        match resolve(&base, &nav_args) {
            Ok(path) => {
                rcmd(&format!("Resolved: {}", path.display()));
                println!("{}", path.display());
            }
            Err(e) => {
                err(&e);
                process::exit(1);
            }
        }
    }
}
