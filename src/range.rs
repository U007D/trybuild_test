#[cfg(test)]
mod unit_tests;

#[derive(Debug)]
pub struct Range<const START: i32, const END: i32>;

impl<const START: i32, const END: i32> Range<START, END> {
    const INVARIANT: () = assert!(START <= END);
}

#[allow(clippy::no_effect, path_statements)]
impl<const START: i32, const END: i32> Drop for Range<START, END> {
    fn drop(&mut self) {
        Self::INVARIANT;
    }
}
