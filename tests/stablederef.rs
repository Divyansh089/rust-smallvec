use {
    smallvec::SmallVec,
    stable_deref_trait::StableDeref,
    std::alloc::System
};

#[test]
fn implemented_without_inline_capacity() {
    fn assert_stable_deref<T: StableDeref>() {}
    assert_stable_deref::<SmallVec<u8, 0>>();
    assert_stable_deref::<SmallVec<String, 0>>();
    assert_stable_deref::<SmallVec<(), 0>>();
    assert_stable_deref::<SmallVec<u8, 0, System>>();
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

#[test]
fn empty_address_survives_move() {
    let vec = SmallVec::<u32, 0>::new();
    let pointer = vec.as_ptr();
    let moved = Box::new(vec);
    assert_eq!(moved.as_ptr(), pointer);
    assert!(moved.is_empty());
}

#[test]
fn zero_sized_address_survives_move() {
    let mut vec = SmallVec::<(), 0>::new();
    vec.extend([(), (), ()]);
    assert!(!vec.spilled());

    let pointer = vec.as_ptr();
    let moved = Box::new(vec);
    assert_eq!(moved.as_ptr(), pointer);
    assert_eq!(moved.len(), 3);
}

#[test]
fn custom_allocator_address_survives_move() {
    let mut vec = SmallVec::<u32, 0, System>::new_in(System);
    vec.extend([1, 2, 3]);
    assert!(vec.spilled());

    let pointer = vec.as_ptr();
    let moved = Box::new(vec);
    assert_eq!(moved.as_ptr(), pointer);
    assert_eq!(**moved, [1, 2, 3]);
}
