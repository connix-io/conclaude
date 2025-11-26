//! Git-ignore pattern matching functionality.
//!
//! This module provides utilities for checking if files are ignored by `.gitignore` patterns.
//! It wraps the `ignore` crate's gitignore functionality with a simple API focused on
//! single-file path checking.
//!
//! # Limitations
//!
//! - Only loads `.gitignore` from the repository root directory. Nested `.gitignore` files
//!   in subdirectories are not currently supported. This may be added in a future version.
//! - Both new file creation (Write) and modification (Edit) of git-ignored files are blocked.
//!   This is intentional - if a file should be ignored, Claude shouldn't create or modify it.
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

/// Find the git repository root by walking up from a starting path.
///
/// Looks for a `.git` directory in the starting path and each parent directory.
/// Returns the directory containing `.git`, or None if not found.
///
/// # Arguments
///
/// * `start_path` - The path to start searching from
///
/// # Returns
///
/// Returns `Some(PathBuf)` with the repository root, or `None` if not in a git repository.
#[must_use]
pub fn find_git_root(start_path: &Path) -> Option<PathBuf> {
    let mut current = if start_path.is_file() {
        start_path.parent()?.to_path_buf()
    } else {
        start_path.to_path_buf()
    };

    loop {
        if current.join(".git").exists() {
            return Some(current);
        }
        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => return None,
        }
    }
}

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
    /// Returns `(bool, Option<String>)` - whether the path is ignored and the matching pattern if found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use conclaude::gitignore::GitIgnoreChecker;
    /// use std::path::Path;
    ///
    /// # fn example() -> anyhow::Result<()> {
    /// let checker = GitIgnoreChecker::new(Path::new("/path/to/repo"))?;
    /// let (is_ignored, pattern) = checker.is_ignored(Path::new("node_modules/foo.js"));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn is_ignored(&self, path: &Path) -> (bool, Option<String>) {
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
        let file_match = self.gitignore.matched(relative_path, false);
        if file_match.is_ignore() {
            let pattern = file_match.inner().map(|g| g.original().to_string());
            return (true, pattern);
        }

        // Then check each parent directory component
        // This handles cases like "node_modules/" ignoring "node_modules/foo.js"
        for ancestor in relative_path.ancestors().skip(1) {
            let dir_match = self.gitignore.matched(ancestor, true);
            if dir_match.is_ignore() {
                let pattern = dir_match.inner().map(|g| g.original().to_string());
                return (true, pattern);
            }
        }

        (false, None)
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
    let (is_ignored, pattern) = checker.is_ignored(path);
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

        // Should be ignored (with pattern)
        let (is_ignored, pattern) = checker.is_ignored(Path::new("node_modules/foo.js"));
        assert!(is_ignored);
        assert_eq!(pattern, Some("node_modules/".to_string()));

        let (is_ignored, pattern) = checker.is_ignored(Path::new("debug.log"));
        assert!(is_ignored);
        assert_eq!(pattern, Some("*.log".to_string()));

        let (is_ignored, _) = checker.is_ignored(Path::new("target/release/app"));
        assert!(is_ignored);

        // Should not be ignored
        let (is_ignored, pattern) = checker.is_ignored(Path::new("src/main.rs"));
        assert!(!is_ignored);
        assert!(pattern.is_none());

        let (is_ignored, _) = checker.is_ignored(Path::new("README.md"));
        assert!(!is_ignored);

        Ok(())
    }

    #[test]
    fn test_no_gitignore_file() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let checker = GitIgnoreChecker::new(temp_dir.path())?;

        // With no .gitignore, nothing should be ignored
        let (is_ignored, _) = checker.is_ignored(Path::new("node_modules/foo.js"));
        assert!(!is_ignored);
        let (is_ignored, _) = checker.is_ignored(Path::new("any/file.txt"));
        assert!(!is_ignored);

        Ok(())
    }

    #[test]
    fn test_negation_patterns() -> Result<()> {
        let temp_dir = create_test_repo("*.log\n!important.log\n")?;
        let checker = GitIgnoreChecker::new(temp_dir.path())?;

        // Regular .log files should be ignored
        let (is_ignored, _) = checker.is_ignored(Path::new("debug.log"));
        assert!(is_ignored);
        let (is_ignored, _) = checker.is_ignored(Path::new("error.log"));
        assert!(is_ignored);

        // important.log should NOT be ignored (negation pattern)
        let (is_ignored, _) = checker.is_ignored(Path::new("important.log"));
        assert!(!is_ignored);

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

        let (is_ignored, _) = checker.is_ignored(Path::new("build/output.js"));
        assert!(is_ignored);
        let (is_ignored, _) = checker.is_ignored(Path::new("build/nested/file.txt"));
        assert!(is_ignored);
        let (is_ignored, _) = checker.is_ignored(Path::new("src/build.rs"));
        assert!(!is_ignored);

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
        let (is_ignored, _) = checker.is_ignored(Path::new("test.log"));
        assert!(is_ignored);
        let (is_ignored, _) = checker.is_ignored(Path::new("test.rs"));
        assert!(!is_ignored);

        Ok(())
    }

    #[test]
    fn test_find_git_root() -> Result<()> {
        // Test that find_git_root correctly locates .git directory
        let temp_dir = TempDir::new()?;
        let git_dir = temp_dir.path().join(".git");
        fs::create_dir(&git_dir)?;

        // From repo root
        assert_eq!(find_git_root(temp_dir.path()), Some(temp_dir.path().to_path_buf()));

        // From subdirectory
        let sub_dir = temp_dir.path().join("src/nested");
        fs::create_dir_all(&sub_dir)?;
        assert_eq!(find_git_root(&sub_dir), Some(temp_dir.path().to_path_buf()));

        // Non-git directory
        let non_git = TempDir::new()?;
        assert_eq!(find_git_root(non_git.path()), None);

        Ok(())
    }
}
