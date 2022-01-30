use trybuild_test::Range;

// 1b of 4. Called from `src/range/unit_tests/trybuild/rs`.  *Should* fail to compile, but doesn't.
// Unit, integration and standalone package versions of same test fail, as expected.
// Next test: `src/range/unit_tests/range_test.rs`
fn main() {
    // Given
    const START: i32 = 100;
    const END: i32 = 1;

    // When
    let _result = Range::<START, END>;    // Invalid `Range`

    // Then
    unreachable!(); // compilation failure
}
