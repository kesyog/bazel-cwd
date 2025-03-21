// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//! Resolve paths in a Bazel-aware manner
//!
//! The `bazel run` command runs the specified executable with the working directory set to the
//! runfiles tree of the binary, not the location where `bazel run` was run from. See
//! <https://bazel.build/docs/user-manual#running-executables> for more.
//!
//! This crates provides a [`BazelPath`] helper trait with an [`unbazelify`][BazelPath::unbazelify]
//! function to resolve paths relative to wherever `bazel run` was run from, allowing for path
//! parsing in a more user-friendly way.
//!
//! ```
//! use std::path::Path;
//! use bazel_cwd::BazelPath;
//!
//! let path = Path::new("foo/bar");
//! let resolved_path = path.unbazelify().unwrap();
//! // resolved_path will be an absolute path, resolving the relative path against the directory
//! // `bazel run` was run in.
//! ```

use std::ffi::OsString;
use std::path::{Path, PathBuf};

pub trait BazelPath {
    /// Resolve the given path to an absolute path in a Bazel-aware manner.
    ///
    /// If the current program was run via `bazel run`, relative paths will be resolved using the
    /// directory the bazel command was run from. Otherwise, it resolves relative paths against the
    /// current working directory. Absolute paths are left as is.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the current working directory could not be determined, e.g. because it
    /// does not exist or there are insufficient permissions to access it.
    fn unbazelify(&self) -> std::io::Result<PathBuf>;
}
impl<T> BazelPath for T
where
    T: AsRef<Path>,
{
    fn unbazelify(&self) -> std::io::Result<PathBuf> {
        unbazelify_impl(self.as_ref())
    }
}

fn unbazelify_impl(path: &Path) -> Result<PathBuf, std::io::Error> {
    if !path.is_relative() {
        return Ok(path.into());
    }
    let root: PathBuf = if let Some(bazel_dir) = std::env::var_os("BUILD_WORKING_DIRECTORY") {
        <OsString as AsRef<Path>>::as_ref(&bazel_dir).to_path_buf()
    } else {
        std::env::current_dir()?
    };
    Ok(root.join(path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absolute_paths() {
        for absolute_path in ["/", "/foo/bar", "/foo/bar/"].into_iter().map(Path::new) {
            assert_eq!(
                absolute_path,
                absolute_path.unbazelify().unwrap(),
                "absolute paths should remain unchanged"
            );
        }
    }

    #[test]
    fn relative_paths() {
        for relative_path in [
            "",
            ".",
            "./foo",
            "foo/",
            "..",
            "../foo",
            "foo/bar/",
            "foo/../bar",
        ]
        .into_iter()
        .map(Path::new)
        {
            assert_ne!(
                relative_path,
                relative_path.unbazelify().unwrap(),
                "relative paths should be resolved to an absolute path"
            );
        }
    }
}
