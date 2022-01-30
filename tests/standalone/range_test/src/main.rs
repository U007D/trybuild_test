use trybuild_test::Range;

// // 4 of 4. `cargo run` from `tests/standalone/range_test` to see compilation fail as fully independent crate (good).
// // Prev Test: `tests/integration_tests.rs`
fn main() {
    // Given
    const START: i32 = 100;
    const END: i32 = 1;

    // When
    let result = Range::<START, END>;    // Invalid `Range`

    // Then
    // compilation failure
}
