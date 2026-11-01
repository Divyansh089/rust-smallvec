use {
    core::iter::Empty,
    defmt::Format,
    smallvec::{
        Drain,
        ExtractIf,
        IntoIter,
        SmallVec,
        SmallVecError,
        Splice
    }
};

fn assert_format<T: Format>() {}

#[test]
fn format() {
    assert_format::<SmallVec<u8, 4>>();
    assert_format::<Drain<'static, u8, 4>>();
    assert_format::<IntoIter<u8, 4>>();
    assert_format::<ExtractIf<'static, u8, 4, fn(&mut u8) -> bool>>();
    assert_format::<Splice<'static, Empty<u8>, 4>>();
    assert_format::<SmallVecError>();
}
