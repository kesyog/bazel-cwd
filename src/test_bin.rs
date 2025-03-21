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
//! Unbazelify the path provided via CLI and prints to stdout.
//!
//! Used for testing behavior with bazel run.

use bazel_cwd::BazelPath;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from(std::env::args_os().nth(1).expect("Path to be provided"));
    println!("{}", path.unbazelify().unwrap().display());
}
