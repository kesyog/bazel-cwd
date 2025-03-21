#!/usr/bin/env bash
# Copyright 2025 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# Exit on error
set -e

pwd=$(pwd)
paths=("foo" "bar" "foo/bar")

bazel build //:test_runner

# Test unbazelification when running within Bazel
for path in "${paths[@]}"; do
  output=$(bazel run //:test_runner -- "$path" 2>/dev/null)

  expected_output="$pwd/$path"
  if [[ "$output" != "$expected_output" ]]; then
    echo "Test failed: Output does not match expected value."
    echo "Expected: $expected_output"
    echo "Actual: $output"
    exit 1
  fi
done

# Test unbazelification when running without Bazel
for path in "${paths[@]}"; do
  output=$("$pwd/bazel-bin/test_runner" "$path")

  expected_output="$pwd/$path"
  if [[ "$output" != "$expected_output" ]]; then
    echo "Test failed: Output does not match expected value."
    echo "Expected: $expected_output"
    echo "Actual: $output"
    exit 1
  fi
done
