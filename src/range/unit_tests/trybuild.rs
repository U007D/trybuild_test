#[allow(unused_imports)]
use super::*;
use std::path::PathBuf;

// // 1a of 4. Uncomment test to see compilation fail as unit test (good).
// // See `/tests/trybuild/range_test.rs` for part 1b.
// // Next test: `src/range/unit_tests/range_test.rs`
#[test]
fn attempt_to_construct_with_invalid_range_fails_tb() {
    // Given
    let try_build = ::trybuild::TestCases::new();
    let path = PathBuf::from("tests/trybuild/range_test.rs");

    // Then
    try_build.compile_fail(path);
}
