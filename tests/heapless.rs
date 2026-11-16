use {
    heapless::{
        Deque,
        Vec as HeaplessVec
    },
    smallvec::SmallVec,
    std::{
        cell::Cell,
        cmp::Ordering,
        rc::Rc
    }
};

/// Counts how many times it has been dropped, to catch leaks and double drops.
struct Tracked(Rc<Cell<usize>>);

impl Drop for Tracked {
    fn drop(&mut self) { self.0.set(self.0.get() + 1); }
}

fn tracked<const N: usize>(count: usize, drops: &Rc<Cell<usize>>) -> HeaplessVec<Tracked, N> {
    (0..count).map(|_| Tracked(drops.clone())).collect()
}

/// A deque holding `[1, 2, 3]` whose contents wrap around the end of its
/// buffer.
fn wrapped_deque() -> Deque<i32, 4> {
    let mut deque = Deque::new();
    for item in [0, 0, 1, 2] {
        deque.push_back(item).unwrap();
    }
    deque.pop_front();
    deque.pop_front();
    deque.push_back(3).unwrap();
    assert!(
        !deque.as_slices().1.is_empty(),
        "contents should wrap around"
    );
    deque
}

#[test]
fn from_vec_stays_inline_or_spills() {
    let small: SmallVec<i32, 4> = HeaplessVec::<i32, 8>::from_iter([1, 2, 3]).into();
    assert!(!small.spilled());
    assert_eq!(&small[..], &[1, 2, 3]);

    let small: SmallVec<i32, 2> = HeaplessVec::<i32, 8>::from_iter([1, 2, 3, 4, 5]).into();
    assert!(small.spilled());
    assert_eq!(&small[..], &[1, 2, 3, 4, 5]);
}

#[test]
fn try_into_vec_fits_or_returns_original() {
    let vec = HeaplessVec::<i32, 4>::try_from(SmallVec::<i32, 4>::from([1, 2, 3, 4])).unwrap();
    assert_eq!(&vec[..], &[1, 2, 3, 4]);
    let vec = HeaplessVec::<i32, 8>::try_from(SmallVec::<i32, 2>::from([1, 2, 3, 4, 5])).unwrap();
    assert_eq!(&vec[..], &[1, 2, 3, 4, 5]);

    // One element too many: the original comes back untouched.
    let error =
        HeaplessVec::<i32, 3>::try_from(SmallVec::<i32, 4>::from([1, 2, 3, 4])).unwrap_err();
    assert!(!error.spilled());
    assert_eq!(&error[..], &[1, 2, 3, 4]);
    let error =
        HeaplessVec::<i32, 4>::try_from(SmallVec::<i32, 2>::from([1, 2, 3, 4, 5])).unwrap_err();
    assert!(error.spilled());
    assert_eq!(&error[..], &[1, 2, 3, 4, 5]);
}

#[test]
fn vec_with_custom_length_type() {
    let small: SmallVec<i32, 2> = HeaplessVec::<i32, 8, u8>::from_iter([1, 2, 3]).into();
    assert_eq!(&small[..], &[1, 2, 3]);
    let vec = HeaplessVec::<i32, 8, u8>::try_from(small.clone()).unwrap();
    assert!(vec == small);
    assert!(small == vec);
}

#[test]
fn vec_conversions_drop_each_element_exactly_once() {
    // `From`, both inline (3 <= 4) and spilled (6 > 4).
    for count in [3, 6] {
        let drops = Rc::new(Cell::new(0));
        let small: SmallVec<Tracked, 4> = tracked::<8>(count, &drops).into();
        assert_eq!(small.spilled(), count > 4);
        assert_eq!(drops.get(), 0, "conversion must not drop elements");
        drop(small);
        assert_eq!(drops.get(), count);
    }

    // `TryFrom` success, both inline and spilled.
    for count in [3, 6] {
        let drops = Rc::new(Cell::new(0));
        let small: SmallVec<Tracked, 4> = tracked::<8>(count, &drops).into();
        let vec = HeaplessVec::<Tracked, 8>::try_from(small).ok().unwrap();
        assert_eq!(drops.get(), 0, "conversion must not drop elements");
        drop(vec);
        assert_eq!(drops.get(), count);
    }

    // `TryFrom` failure hands every element back.
    let drops = Rc::new(Cell::new(0));
    let small: SmallVec<Tracked, 2> = tracked::<8>(5, &drops).into();
    let error = HeaplessVec::<Tracked, 3>::try_from(small).err().unwrap();
    assert_eq!(drops.get(), 0, "failed conversion must not drop elements");
    drop(error);
    assert_eq!(drops.get(), 5);
}

#[test]
fn vec_zero_sized_items() {
    let vec: HeaplessVec<(), 8> = std::iter::repeat_n((), 5).collect();
    let small: SmallVec<(), 2> = vec.into();
    assert_eq!(small.len(), 5);

    assert!(HeaplessVec::<(), 4>::try_from(small.clone()).is_err());
    assert_eq!(HeaplessVec::<(), 5>::try_from(small).unwrap().len(), 5);
}

#[test]
fn vec_eq_across_capacities_and_item_types() {
    let small = SmallVec::<i32, 2>::from([1, 2, 3]);
    let vec = HeaplessVec::<i32, 8>::from_iter([1, 2, 3]);
    assert!(small == vec);
    assert!(vec == small);
    assert!(small != HeaplessVec::<i32, 8>::from_iter([1, 2, 4]));
    let shorter = SmallVec::<i32, 2>::from([1, 2]);
    assert!(vec != shorter);

    // `Item: PartialEq<Other>` with different types.
    let small: SmallVec<String, 2> = SmallVec::from(["a".to_string(), "b".to_string()]);
    let vec = HeaplessVec::<&str, 4>::from_iter(["a", "b"]);
    assert!(small == vec);
    assert!(vec == small);
}

#[test]
fn vec_ordering_in_both_directions() {
    let small = SmallVec::<i32, 2>::from([1, 2, 3]);
    let vec = HeaplessVec::<i32, 8>::from_iter([1, 2, 4]);
    assert!(small < vec);
    assert!(vec > small);
    assert_eq!(small.partial_cmp(&vec), Some(Ordering::Less));
    assert_eq!(vec.partial_cmp(&small), Some(Ordering::Greater));
}

#[test]
fn from_deque_keeps_order() {
    let small: SmallVec<i32, 2> = wrapped_deque().into();
    assert!(small.spilled());
    assert_eq!(&small[..], &[1, 2, 3]);

    let small: SmallVec<i32, 4> = wrapped_deque().into();
    assert!(!small.spilled());
    assert_eq!(&small[..], &[1, 2, 3]);
}

#[test]
fn try_into_deque_fits_or_returns_original() {
    let deque = Deque::<i32, 4>::try_from(SmallVec::<i32, 2>::from([1, 2, 3, 4])).unwrap();
    assert_eq!(deque.iter().copied().collect::<Vec<_>>(), [1, 2, 3, 4]);

    // One element too many: the original comes back untouched.
    let error = Deque::<i32, 3>::try_from(SmallVec::<i32, 2>::from([1, 2, 3, 4])).unwrap_err();
    assert_eq!(&error[..], &[1, 2, 3, 4]);
}

#[test]
fn deque_conversions_drop_each_element_exactly_once() {
    // `TryFrom` success, then `From` back.
    let drops = Rc::new(Cell::new(0));
    let small: SmallVec<Tracked, 2> = tracked::<8>(5, &drops).into();
    let deque = Deque::<Tracked, 8>::try_from(small).ok().unwrap();
    assert_eq!(drops.get(), 0, "conversion must not drop elements");
    let small: SmallVec<Tracked, 2> = deque.into();
    assert_eq!(drops.get(), 0, "conversion must not drop elements");
    drop(small);
    assert_eq!(drops.get(), 5);

    // `TryFrom` failure hands every element back.
    let drops = Rc::new(Cell::new(0));
    let small: SmallVec<Tracked, 2> = tracked::<8>(5, &drops).into();
    let error = Deque::<Tracked, 3>::try_from(small).err().unwrap();
    assert_eq!(drops.get(), 0, "failed conversion must not drop elements");
    drop(error);
    assert_eq!(drops.get(), 5);
}

#[test]
fn deque_eq_with_wrapped_contents() {
    let deque = wrapped_deque();
    let small = SmallVec::<i32, 2>::from([1, 2, 3]);
    assert!(small == deque);
    assert!(deque == small);
    assert!(SmallVec::<i32, 2>::from([1, 2]) != deque);
    assert!(deque != SmallVec::<i32, 2>::from([1, 2, 3, 4]));
    assert!(deque != SmallVec::<i32, 2>::from([1, 2, 4]));
}

#[test]
fn deque_ordering_in_both_directions() {
    let deque = wrapped_deque();
    let small = SmallVec::<i32, 2>::from([1, 2, 4]);
    assert!(deque < small);
    assert!(small > deque);
    assert_eq!(deque.partial_cmp(&small), Some(Ordering::Less));
    assert_eq!(small.partial_cmp(&deque), Some(Ordering::Greater));
}

#[cfg(feature = "equivalent")]
#[test]
fn equivalent_and_comparable() {
    use equivalent::{
        Comparable,
        Equivalent
    };

    let small = SmallVec::<i32, 2>::from([1, 2, 3]);
    let vec = HeaplessVec::<i32, 8>::from_iter([1, 2, 3]);
    let deque = wrapped_deque();

    assert!(small.equivalent(&vec));
    assert!(vec.equivalent(&small));
    assert!(small.equivalent(&deque));
    assert!(deque.equivalent(&small));
    assert!(!SmallVec::<i32, 2>::from([1, 2]).equivalent(&deque));

    assert_eq!(small.compare(&vec), Ordering::Equal);
    assert_eq!(vec.compare(&small), Ordering::Equal);
    assert_eq!(small.compare(&deque), Ordering::Equal);
    assert_eq!(deque.compare(&small), Ordering::Equal);
    assert_eq!(
        SmallVec::<i32, 2>::from([1, 2]).compare(&deque),
        Ordering::Less
    );
    assert_eq!(
        deque.compare(&SmallVec::<i32, 2>::from([1, 2, 4])),
        Ordering::Less
    );
}
