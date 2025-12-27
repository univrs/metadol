//! DOL Syntax Migration Tool
//!
//! Migrates DOL files from v0.2.x syntax to v0.3.0 syntax.
//!
//! # Usage
//!
//! ```bash
//! # Migrate a single file
//! dol-migrate --from 0.2 --to 0.3 path/to/file.dol
//!
//! # Migrate a directory
//! dol-migrate --from 0.2 --to 0.3 src/
//!
//! # Preview changes without applying
//! dol-migrate --from 0.2 --to 0.3 --dry-run src/
//! ```

use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

/// Migration rules from v0.2 to v0.3
const MIGRATIONS_V02_TO_V03: &[(&str, &str)] = &[
    // Bindings: let → val, let mut → var
    (r"\blet\s+mut\s+(\w+)", "var $1"),
    (r"\blet\s+(\w+)", "val $1"),
    // Quantifiers: each/all → forall
    (r"\beach\s+(\w+)\s+in\b", "forall $1 in"),
    (r"\ball\s+(\w+)\s+in\b", "forall $1 in"),
    // Module: module → mod
    (r"\bmodule\s+", "mod "),
    // Negation: never → not
    (r"\bnever\s+", "not "),
    // Inheritance: derives from → extends
    (r"\bderives\s+from\s+", "extends "),
    // Equality: matches → ==
    (r"\bmatches\s+", "== "),
    // Test syntax: given → val (in test context)
    (r"\bgiven\s+(\w+)\s*=", "val $1 ="),
    // then → assert (in test context)
    (r"\bthen\s+", "assert "),
];

/// Migration result
#[derive(Debug)]
pub struct MigrationResult {
    pub path: PathBuf,
    pub original: String,
    pub migrated: String,
    pub changes: Vec<String>,
}

impl MigrationResult {
    pub fn has_changes(&self) -> bool {
        self.original != self.migrated
    }
}

/// Apply migration rules to source text
pub fn migrate_source(source: &str, from: &str, to: &str) -> (String, Vec<String>) {
    if from != "0.2" || to != "0.3" {
        return (source.to_string(), vec![]);
    }

    let mut result = source.to_string();
    let mut changes = Vec::new();

    for (pattern, replacement) in MIGRATIONS_V02_TO_V03 {
        let re = Regex::new(pattern).expect("Invalid regex pattern");
        if re.is_match(&result) {
            changes.push(format!("{} -> {}", pattern, replacement));
            result = re.replace_all(&result, *replacement).to_string();
        }
    }

    (result, changes)
}

/// Migrate a single file
pub fn migrate_file(path: &Path, from: &str, to: &str) -> std::io::Result<MigrationResult> {
    let original = fs::read_to_string(path)?;
    let (migrated, changes) = migrate_source(&original, from, to);

    Ok(MigrationResult {
        path: path.to_path_buf(),
        original,
        migrated,
        changes,
    })
}

/// Migrate all .dol files in a directory
pub fn migrate_directory(
    dir: &Path,
    from: &str,
    to: &str,
) -> std::io::Result<Vec<MigrationResult>> {
    let mut results = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            results.extend(migrate_directory(&path, from, to)?);
        } else if path.extension().is_some_and(|e| e == "dol") {
            results.push(migrate_file(&path, from, to)?);
        }
    }

    Ok(results)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Parse arguments
    let mut from = "0.2";
    let mut to = "0.3";
    let mut dry_run = false;
    let mut paths: Vec<String> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--from" => {
                i += 1;
                from = args.get(i).map(|s| s.as_str()).unwrap_or("0.2");
            }
            "--to" => {
                i += 1;
                to = args.get(i).map(|s| s.as_str()).unwrap_or("0.3");
            }
            "--dry-run" => {
                dry_run = true;
            }
            "--help" | "-h" => {
                println!("DOL Syntax Migration Tool");
                println!();
                println!("Usage: dol-migrate [OPTIONS] <PATH>...");
                println!();
                println!("Options:");
                println!("  --from <VERSION>  Source version (default: 0.2)");
                println!("  --to <VERSION>    Target version (default: 0.3)");
                println!("  --dry-run         Preview changes without writing");
                println!("  -h, --help        Show this help message");
                println!();
                println!("Examples:");
                println!("  dol-migrate --from 0.2 --to 0.3 src/");
                println!("  dol-migrate --dry-run src/foo.dol");
                return;
            }
            path => {
                paths.push(path.to_string());
            }
        }
        i += 1;
    }

    if paths.is_empty() {
        eprintln!("Error: No paths specified");
        eprintln!("Usage: dol-migrate [OPTIONS] <PATH>...");
        std::process::exit(1);
    }

    println!("Migrating from {} to {}...", from, to);
    if dry_run {
        println!("(dry run - no files will be modified)");
    }
    println!();

    let mut total_files = 0;
    let mut changed_files = 0;

    for path_str in &paths {
        let path = Path::new(path_str);

        let results = if path.is_dir() {
            migrate_directory(path, from, to)
        } else {
            migrate_file(path, from, to).map(|r| vec![r])
        };

        match results {
            Ok(results) => {
                for result in results {
                    total_files += 1;

                    if result.has_changes() {
                        changed_files += 1;
                        println!("[ok] {}", result.path.display());
                        for change in &result.changes {
                            println!("  - {}", change);
                        }

                        if !dry_run {
                            if let Err(e) = fs::write(&result.path, &result.migrated) {
                                eprintln!("  Error writing file: {}", e);
                            }
                        }
                    } else {
                        println!("  {} (no changes)", result.path.display());
                    }
                }
            }
            Err(e) => {
                eprintln!("Error processing {}: {}", path_str, e);
            }
        }
    }

    println!();
    println!(
        "Summary: {} files processed, {} files changed",
        total_files, changed_files
    );

    if dry_run && changed_files > 0 {
        println!("Run without --dry-run to apply changes.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrate_let_to_val() {
        let (result, changes) = migrate_source("let x = 42", "0.2", "0.3");
        assert_eq!(result, "val x = 42");
        assert!(!changes.is_empty());
    }

    #[test]
    fn test_migrate_let_mut_to_var() {
        let (result, _) = migrate_source("let mut counter = 0", "0.2", "0.3");
        assert_eq!(result, "var counter = 0");
    }

    #[test]
    fn test_migrate_each_to_forall() {
        let (result, _) = migrate_source("each item in items", "0.2", "0.3");
        assert_eq!(result, "forall item in items");
    }

    #[test]
    fn test_migrate_module_to_mod() {
        let (result, _) = migrate_source("module dol.parser @ 0.3.0", "0.2", "0.3");
        assert_eq!(result, "mod dol.parser @ 0.3.0");
    }

    #[test]
    fn test_migrate_never_to_not() {
        let (result, _) = migrate_source("value never overflows", "0.2", "0.3");
        assert_eq!(result, "value not overflows");
    }

    #[test]
    fn test_migrate_derives_from_to_extends() {
        let (result, _) = migrate_source("container derives from image", "0.2", "0.3");
        assert_eq!(result, "container extends image");
    }

    #[test]
    fn test_no_changes_for_same_version() {
        let (result, changes) = migrate_source("let x = 42", "0.3", "0.3");
        assert_eq!(result, "let x = 42");
        assert!(changes.is_empty());
    }

    #[test]
    fn test_full_migration() {
        let source = r#"
module greeting.service @ 0.1.0

gene greeting.entity {
    entity has identity
    each greeting in greetings {
        greeting never empty
    }
}

test "greeting works" {
    given g = Greeting.new("Hello")
    then g.delivered == true
}
"#;
        let (result, _) = migrate_source(source, "0.2", "0.3");

        assert!(result.contains("mod greeting.service"));
        assert!(result.contains("forall greeting in greetings"));
        assert!(result.contains("greeting not empty"));
        assert!(result.contains("val g ="));
        assert!(result.contains("assert g.delivered"));
    }
}
