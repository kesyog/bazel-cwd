# bazel-cwd

Resolve paths in a Bazel-aware manner.

As described in <https://bazel.build/docs/user-manual#running-executables>, the `bazel run` command
runs from the runfiles tree of the binary, not the location where `bazel run` was run from. This
must be accounted for in order to resolve relative paths in a user-friendly manner.

Bazel does expose the "correct" path via an environment variable. This crates provides a helper
trait `BazelPath` and an `unbazelify` function to resolve paths the way you'd probably expect. It
has reasonable fallback behavior when passed an absolute path or when used from outside of
`bazel run`.

```rust
use std::path::Path;
use bazel_cwd::BazelPath;

let path = Path::new("foo/bar");
let resolved_path = path.unbazelify().unwrap();
// resolved_path will be an absolute path, resolving the relative path against the directory
// `bazel run` was run in.
```

This is not an officially supported Google product.
