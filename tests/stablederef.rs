use {
    smallvec::SmallVec,
    stable_deref_trait::StableDeref
};

#[test]
fn implemented_without_inline_capacity() {
    fn assert_stable_deref<T: StableDeref>() {}
    assert_stable_deref::<SmallVec<u8, 0>>();
    assert_stable_deref::<SmallVec<String, 0>>();
    assert_stable_deref::<SmallVec<(), 0>>();
}

#[test]
fn address_survives_move() {
    let mut vec = SmallVec::<u32, 0>::new();
    vec.extend([1, 2, 3]);
    assert!(vec.spilled());

    let pointer = vec.as_ptr();
    let moved = Box::new(vec);
    assert_eq!(moved.as_ptr(), pointer);
    assert_eq!(**moved, [1, 2, 3]);
}
