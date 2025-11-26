//! Git-ignore pattern matching functionality.
//!
//! This module provides utilities for checking if files are ignored by `.gitignore` patterns.
//! It wraps the `ignore` crate's gitignore functionality with a simple API focused on
//! single-file path checking.
//!
//! # Examples
//!
//! ```rust
//! use conclaude::gitignore::is_path_git_ignored;
//! use std::path::Path;
//!
//! # fn example() -> anyhow::Result<()> {
//! let repo_root = Path::new("/path/to/repo");
//! let (is_ignored, pattern) = is_path_git_ignored(
//!     Path::new("node_modules/foo.js"),
//!     repo_root
//! )?;
//!
//! if is_ignored {
//!     println!("File is ignored, matched pattern: {:?}", pattern);
//! }
//! # Ok(())
//! # }
//! ```

use anyhow::{Context, Result};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::{Path, PathBuf};

/// A checker for git-ignore patterns.
///
/// Wraps the `ignore::gitignore::Gitignore` to provide a simple API for checking
/// if paths match gitignore patterns.
pub struct GitIgnoreChecker {
    gitignore: Gitignore,
    repo_root: PathBuf,
}

impl GitIgnoreChecker {
    /// Creates a new `GitIgnoreChecker` by loading `.gitignore` from the repository root.
    ///
    /// # Arguments
    ///
    /// * `repo_root` - The root directory of the git repository containing `.gitignore`
    ///
    /// # Returns
    ///
    /// Returns a `GitIgnoreChecker` instance. If no `.gitignore` file exists or it cannot
    /// be parsed, the checker will treat all files as non-ignored (logs warnings for errors).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use conclaude::gitignore::GitIgnoreChecker;
    /// use std::path::Path;
    ///
    /// # fn example() -> anyhow::Result<()> {
    /// let checker = GitIgnoreChecker::new(Path::new("/path/to/repo"))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(repo_root: &Path) -> Result<Self> {
        let gitignore_path = repo_root.join(".gitignore");

        let mut builder = GitignoreBuilder::new(repo_root);

        // If .gitignore exists, try to add it
        if gitignore_path.exists() {
            if let Some(e) = builder.add(&gitignore_path) {
                // Log warning but continue with empty gitignore
                eprintln!(
                    "Warning: Failed to parse .gitignore at {}: {}. Treating all files as non-ignored.",
                    gitignore_path.display(),
                    e
                );
            }
        }

        let gitignore = builder
            .build()
            .context("Failed to build gitignore matcher")?;

        Ok(Self {
            gitignore,
            repo_root: repo_root.to_path_buf(),
        })
    }

    /// Checks if a path matches gitignore patterns.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path to check (should be relative to repository root, but absolute paths work too)
    ///
    /// # Returns
    ///
    /// Returns `true` if the path is ignored by gitignore patterns, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use conclaude::gitignore::GitIgnoreChecker;
    /// use std::path::Path;
    ///
    /// # fn example() -> anyhow::Result<()> {
    /// let checker = GitIgnoreChecker::new(Path::new("/path/to/repo"))?;
    /// let is_ignored = checker.is_ignored(Path::new("node_modules/foo.js"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_ignored(&self, path: &Path) -> bool {
        // Convert to relative path if it's absolute
        let relative_path = if path.is_absolute() {
            match path.strip_prefix(&self.repo_root) {
                Ok(rel) => rel,
                Err(_) => path, // If not under repo root, use as-is
            }
        } else {
            path
        };

        // Check if the path matches gitignore patterns
        // We need to check both as a file and as a directory, because:
        // 1. node_modules/ only matches directories
        // 2. *.log matches files
        // We check the parent components as directories to handle nested paths like node_modules/foo.js

        // First check if this exact path is ignored as a file
        if self.gitignore.matched(relative_path, false).is_ignore() {
            return true;
        }

        // Then check each parent directory component
        // This handles cases like "node_modules/" ignoring "node_modules/foo.js"
        for ancestor in relative_path.ancestors().skip(1) {
            if self.gitignore.matched(ancestor, true).is_ignore() {
                return true;
            }
        }

        false
    }
}

/// Convenience function to check if a path is git-ignored.
///
/// This is a simplified API that creates a `GitIgnoreChecker` and checks a single path.
/// Use this for one-off checks. For multiple checks, create a `GitIgnoreChecker` instance
/// and reuse it.
///
/// # Arguments
///
/// * `path` - The file path to check
/// * `repo_root` - The root directory of the git repository
///
/// # Returns
///
/// Returns a tuple of:
/// * `bool` - Whether the path is ignored
/// * `Option<String>` - The matching pattern (if ignored), useful for error messages
///
/// # Examples
///
/// ```rust
/// use conclaude::gitignore::is_path_git_ignored;
/// use std::path::Path;
///
/// # fn example() -> anyhow::Result<()> {
/// let (is_ignored, pattern) = is_path_git_ignored(
///     Path::new("node_modules/foo.js"),
///     Path::new("/path/to/repo")
/// )?;
///
/// if is_ignored {
///     println!("File is ignored by pattern: {:?}", pattern);
/// }
/// # Ok(())
/// # }
/// ```
pub fn is_path_git_ignored(path: &Path, repo_root: &Path) -> Result<(bool, Option<String>)> {
    let checker = GitIgnoreChecker::new(repo_root)?;
    let is_ignored = checker.is_ignored(path);

    // If ignored, try to get the matching pattern for better error messages
    let pattern = if is_ignored {
        // The ignore crate doesn't directly expose which pattern matched,
        // but we can provide the gitignore file location as context
        Some(format!("matched by {}/.gitignore", repo_root.display()))
    } else {
        None
    };

    Ok((is_ignored, pattern))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Helper to create a test repository with a .gitignore file
    fn create_test_repo(gitignore_content: &str) -> Result<TempDir> {
        let temp_dir = TempDir::new()?;
        let gitignore_path = temp_dir.path().join(".gitignore");
        fs::write(&gitignore_path, gitignore_content)?;
        Ok(temp_dir)
    }

    #[test]
    fn test_basic_gitignore_patterns() -> Result<()> {
        let temp_dir = create_test_repo("node_modules/\n*.log\ntarget/\n")?;
        let checker = GitIgnoreChecker::new(temp_dir.path())?;

        // Should be ignored
        assert!(checker.is_ignored(Path::new("node_modules/foo.js")));
        assert!(checker.is_ignored(Path::new("debug.log")));
        assert!(checker.is_ignored(Path::new("target/release/app")));

        // Should not be ignored
        assert!(!checker.is_ignored(Path::new("src/main.rs")));
        assert!(!checker.is_ignored(Path::new("README.md")));

        Ok(())
    }

    #[test]
    fn test_no_gitignore_file() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let checker = GitIgnoreChecker::new(temp_dir.path())?;

        // With no .gitignore, nothing should be ignored
        assert!(!checker.is_ignored(Path::new("node_modules/foo.js")));
        assert!(!checker.is_ignored(Path::new("any/file.txt")));

        Ok(())
    }

    #[test]
    fn test_negation_patterns() -> Result<()> {
        let temp_dir = create_test_repo("*.log\n!important.log\n")?;
        let checker = GitIgnoreChecker::new(temp_dir.path())?;

        // Regular .log files should be ignored
        assert!(checker.is_ignored(Path::new("debug.log")));
        assert!(checker.is_ignored(Path::new("error.log")));

        // important.log should NOT be ignored (negation pattern)
        assert!(!checker.is_ignored(Path::new("important.log")));

        Ok(())
    }

    #[test]
    fn test_is_path_git_ignored_convenience_function() -> Result<()> {
        let temp_dir = create_test_repo("*.tmp\ncache/\n")?;

        let (is_ignored, pattern) = is_path_git_ignored(
            Path::new("file.tmp"),
            temp_dir.path()
        )?;
        assert!(is_ignored);
        assert!(pattern.is_some());

        let (is_ignored, pattern) = is_path_git_ignored(
            Path::new("file.txt"),
            temp_dir.path()
        )?;
        assert!(!is_ignored);
        assert!(pattern.is_none());

        Ok(())
    }

    #[test]
    fn test_subdirectory_patterns() -> Result<()> {
        let temp_dir = create_test_repo("build/\n")?;
        let checker = GitIgnoreChecker::new(temp_dir.path())?;

        assert!(checker.is_ignored(Path::new("build/output.js")));
        assert!(checker.is_ignored(Path::new("build/nested/file.txt")));
        assert!(!checker.is_ignored(Path::new("src/build.rs")));

        Ok(())
    }

    #[test]
    fn test_corrupt_gitignore_handling() -> Result<()> {
        // Create a temp directory with an invalid/corrupt .gitignore
        // The ignore crate is fairly permissive, so we'll just test that
        // the checker handles it gracefully
        let temp_dir = TempDir::new()?;
        let gitignore_path = temp_dir.path().join(".gitignore");

        // Write some edge case content (most patterns will parse fine though)
        fs::write(&gitignore_path, "node_modules/\n\n# comment\n*.log\n")?;

        // Should not panic or error
        let checker = GitIgnoreChecker::new(temp_dir.path())?;

        // Should still work with valid patterns
        assert!(checker.is_ignored(Path::new("test.log")));
        assert!(!checker.is_ignored(Path::new("test.rs")));

        Ok(())
    }
}
