use {
    core::hint::black_box,
    iai_callgrind::{
        library_benchmark,
        library_benchmark_group,
        main
    },
    smallvec::SmallVec
};

const INLINE_CAP: usize = 16;
const SPILLED_COUNT: usize = 100;

// =========================================================================
// PUSH
// =========================================================================

#[library_benchmark]
fn bench_smallvec_push_inline() -> SmallVec<u64, INLINE_CAP> {
    let mut v = SmallVec::<u64, INLINE_CAP>::new();
    for i in 0..(INLINE_CAP as u64) {
        v.push(black_box(i));
    }
    v
}

#[library_benchmark]
fn bench_smallvec_push_spilled() -> SmallVec<u64, INLINE_CAP> {
    let mut v = SmallVec::<u64, INLINE_CAP>::new();
    for i in 0..(SPILLED_COUNT as u64) {
        v.push(black_box(i));
    }
    v
}

#[library_benchmark]
fn bench_vec_push() -> Vec<u64> {
    let mut v = Vec::with_capacity(INLINE_CAP);
    for i in 0..(INLINE_CAP as u64) {
        v.push(black_box(i));
    }
    v
}

#[library_benchmark]
fn bench_vec_push_with_spilled_capacity() -> Vec<u64> {
    let mut v = Vec::new();
    for i in 0..(SPILLED_COUNT as u64) {
        v.push(black_box(i));
    }
    v
}

// =========================================================================
// POP
// =========================================================================

#[library_benchmark]
fn bench_smallvec_pop_inline() -> u64 {
    let mut v = SmallVec::<u64, INLINE_CAP>::new();
    for i in 0..(INLINE_CAP as u64) {
        v.push(black_box(i));
    }
    let mut sum = 0;
    while let Some(val) = v.pop() {
        sum += black_box(val);
    }
    sum
}

#[library_benchmark]
fn bench_smallvec_pop_spilled() -> u64 {
    let mut v = SmallVec::<u64, INLINE_CAP>::new();
    for i in 0..(SPILLED_COUNT as u64) {
        v.push(black_box(i));
    }
    let mut sum = 0;
    while let Some(val) = v.pop() {
        sum += black_box(val);
    }
    sum
}

#[library_benchmark]
fn bench_vec_pop() -> u64 {
    let mut v = Vec::new();
    for i in 0..(SPILLED_COUNT as u64) {
        v.push(black_box(i));
    }
    let mut sum = 0;
    while let Some(val) = v.pop() {
        sum += black_box(val);
    }
    sum
}

// =========================================================================
// INSERT & REMOVE
// =========================================================================

#[library_benchmark]
fn bench_smallvec_insert_remove_inline() -> SmallVec<u64, INLINE_CAP> {
    let mut v = SmallVec::<u64, INLINE_CAP>::new();
    for i in 0..((INLINE_CAP - 1) as u64) {
        v.push(black_box(i));
    }
    v.insert(black_box(4), black_box(999));
    let _ = v.remove(black_box(4));
    v
}

#[library_benchmark]
fn bench_smallvec_insert_remove_spilled() -> SmallVec<u64, INLINE_CAP> {
    let mut v = SmallVec::<u64, INLINE_CAP>::new();
    for i in 0..(SPILLED_COUNT as u64) {
        v.push(black_box(i));
    }
    v.insert(black_box(50), black_box(999));
    let _ = v.remove(black_box(50));
    v
}

#[library_benchmark]
fn bench_vec_insert_remove() -> Vec<u64> {
    let mut v = Vec::new();
    for i in 0..(SPILLED_COUNT as u64) {
        v.push(black_box(i));
    }
    v.insert(black_box(50), black_box(999));
    let _ = v.remove(black_box(50));
    v
}

// =========================================================================
// EXTEND & FROM_SLICE
// =========================================================================

#[library_benchmark]
fn bench_smallvec_from_slice_inline() -> SmallVec<u64, INLINE_CAP> {
    let data = [42u64; INLINE_CAP];
    SmallVec::<u64, INLINE_CAP>::from(black_box(data.as_slice()))
}

#[library_benchmark]
fn bench_smallvec_from_slice_spilled() -> SmallVec<u64, INLINE_CAP> {
    let data = [42u64; SPILLED_COUNT];
    SmallVec::<u64, INLINE_CAP>::from(black_box(data.as_slice()))
}

#[library_benchmark]
fn bench_smallvec_extend_from_slice_inline() -> SmallVec<u64, INLINE_CAP> {
    let mut v = SmallVec::<u64, INLINE_CAP>::new();
    let data = [42u64; INLINE_CAP];
    v.extend_from_slice_copy(black_box(&data));
    v
}

#[library_benchmark]
fn bench_smallvec_extend_from_slice_spilled() -> SmallVec<u64, INLINE_CAP> {
    let mut v = SmallVec::<u64, INLINE_CAP>::new();
    let data = [42u64; SPILLED_COUNT];
    v.extend_from_slice_copy(black_box(&data));
    v
}

// The generic `Extend<&Item>` path, which clones element by element.
#[library_benchmark]
fn bench_smallvec_extend_ref_inline() -> SmallVec<u64, INLINE_CAP> {
    let mut v = SmallVec::<u64, INLINE_CAP>::new();
    let data = [42u64; INLINE_CAP];
    v.extend(black_box(&data));
    v
}

#[library_benchmark]
fn bench_smallvec_extend_ref_spilled() -> SmallVec<u64, INLINE_CAP> {
    let mut v = SmallVec::<u64, INLINE_CAP>::new();
    let data = [42u64; SPILLED_COUNT];
    v.extend(black_box(&data));
    v
}

#[library_benchmark]
fn bench_smallvec_insert_from_slice_inline() -> SmallVec<u64, INLINE_CAP> {
    let mut v: SmallVec<u64, INLINE_CAP> = smallvec::from_elem(0, INLINE_CAP / 2);
    let data = [42u64; INLINE_CAP / 2];
    v.insert_from_slice_copy(black_box(INLINE_CAP / 4), black_box(&data));
    v
}

#[library_benchmark]
fn bench_smallvec_insert_from_slice_spilled() -> SmallVec<u64, INLINE_CAP> {
    let mut v: SmallVec<u64, INLINE_CAP> = smallvec::from_elem(0, SPILLED_COUNT / 2);
    let data = [42u64; SPILLED_COUNT / 2];
    v.insert_from_slice_copy(black_box(SPILLED_COUNT / 4), black_box(&data));
    v
}

#[library_benchmark]
fn bench_vec_from_slice() -> Vec<u64> {
    let data = [42u64; SPILLED_COUNT];
    Vec::from(black_box(data.as_slice()))
}

#[library_benchmark]
fn bench_vec_extend_from_slice() -> Vec<u64> {
    let mut v = Vec::new();
    let data = [42u64; SPILLED_COUNT];
    v.extend_from_slice(black_box(&data));
    v
}

// =========================================================================
// ARRAY / BUF CONVERSION
// =========================================================================

#[library_benchmark]
fn bench_smallvec_from_array_inline() -> SmallVec<u64, INLINE_CAP> {
    SmallVec::<u64, INLINE_CAP>::from(black_box([1u64; INLINE_CAP]))
}

#[library_benchmark]
fn bench_smallvec_from_array_spilled() -> SmallVec<u64, INLINE_CAP> {
    SmallVec::<u64, INLINE_CAP>::from(black_box([1u64; SPILLED_COUNT]))
}

#[library_benchmark]
fn bench_vec_from_array() -> Vec<u64> { Vec::from(black_box([1u64; SPILLED_COUNT])) }

#[library_benchmark]
fn bench_smallvec_from_elem_spilled() -> SmallVec<u64, INLINE_CAP> {
    smallvec::from_elem(black_box(1u64), black_box(SPILLED_COUNT))
}

// =========================================================================
// ITERATION & DRAIN
// =========================================================================

#[library_benchmark]
fn bench_smallvec_into_iter_inline() -> u64 {
    let mut v = SmallVec::<u64, INLINE_CAP>::new();
    for i in 0..(INLINE_CAP as u64) {
        v.push(black_box(i));
    }
    v.into_iter().fold(0, |acc, x| acc + black_box(x))
}

#[library_benchmark]
fn bench_smallvec_into_iter_spilled() -> u64 {
    let mut v = SmallVec::<u64, INLINE_CAP>::new();
    for i in 0..(SPILLED_COUNT as u64) {
        v.push(black_box(i));
    }
    v.into_iter().fold(0, |acc, x| acc + black_box(x))
}

#[library_benchmark]
fn bench_smallvec_drain_inline() -> u64 {
    let mut v = SmallVec::<u64, INLINE_CAP>::new();
    for i in 0..(INLINE_CAP as u64) {
        v.push(black_box(i));
    }
    let sum = v.drain(..).fold(0, |acc, x| acc + black_box(x));
    black_box(v);
    sum
}

#[library_benchmark]
fn bench_smallvec_drain_spilled() -> u64 {
    let mut v = SmallVec::<u64, INLINE_CAP>::new();
    for i in 0..(SPILLED_COUNT as u64) {
        v.push(black_box(i));
    }
    let sum = v.drain(..).fold(0, |acc, x| acc + black_box(x));
    black_box(v);
    sum
}

#[library_benchmark]
fn bench_vec_into_iter() -> u64 {
    let mut v = Vec::new();
    for i in 0..(SPILLED_COUNT as u64) {
        v.push(black_box(i));
    }
    v.into_iter().fold(0, |acc, x| acc + black_box(x))
}

#[library_benchmark]
fn bench_vec_drain() -> u64 {
    let mut v = Vec::new();
    for i in 0..(SPILLED_COUNT as u64) {
        v.push(black_box(i));
    }
    let sum = v.drain(..).fold(0, |acc, x| acc + black_box(x));
    black_box(v);
    sum
}

// =========================================================================
// RETAIN MUT
// =========================================================================

#[library_benchmark]
fn bench_smallvec_retain_mut_inline() -> SmallVec<u64, INLINE_CAP> {
    let mut v = SmallVec::<u64, INLINE_CAP>::new();
    for i in 0..(INLINE_CAP as u64) {
        v.push(black_box(i));
    }
    v.retain_mut(|x| *x % 2 == 0);
    v
}

#[library_benchmark]
fn bench_smallvec_retain_mut_spilled() -> SmallVec<u64, INLINE_CAP> {
    let mut v = SmallVec::<u64, INLINE_CAP>::new();
    for i in 0..(SPILLED_COUNT as u64) {
        v.push(black_box(i));
    }
    v.retain_mut(|x| *x % 2 == 0);
    v
}

#[library_benchmark]
fn bench_vec_retain_mut() -> Vec<u64> {
    let mut v = Vec::new();
    for i in 0..(SPILLED_COUNT as u64) {
        v.push(black_box(i));
    }
    v.retain_mut(|x| *x % 2 == 0);
    v
}

// =========================================================================
// CLONE
// =========================================================================

fn smallvec_with(count: usize) -> SmallVec<u64, INLINE_CAP> {
    (0..count as u64).map(black_box).collect()
}

fn vec_with(count: usize) -> Vec<u64> { (0..count as u64).map(black_box).collect() }

#[library_benchmark]
fn bench_smallvec_clone_inline() -> SmallVec<u64, INLINE_CAP> {
    let v = smallvec_with(INLINE_CAP);
    black_box(&v).clone()
}

#[library_benchmark]
fn bench_smallvec_clone_spilled() -> SmallVec<u64, INLINE_CAP> {
    let v = smallvec_with(SPILLED_COUNT);
    black_box(&v).clone()
}

#[library_benchmark]
fn bench_vec_clone() -> Vec<u64> {
    let v = vec_with(SPILLED_COUNT);
    black_box(&v).clone()
}

// =========================================================================
// TRUNCATE & CLEAR
// =========================================================================

#[library_benchmark]
fn bench_smallvec_truncate_inline() -> SmallVec<u64, INLINE_CAP> {
    let mut v = smallvec_with(INLINE_CAP);
    v.truncate(black_box(INLINE_CAP / 2));
    v
}

#[library_benchmark]
fn bench_smallvec_truncate_spilled() -> SmallVec<u64, INLINE_CAP> {
    let mut v = smallvec_with(SPILLED_COUNT);
    v.truncate(black_box(SPILLED_COUNT / 2));
    v
}

#[library_benchmark]
fn bench_vec_truncate() -> Vec<u64> {
    let mut v = vec_with(SPILLED_COUNT);
    v.truncate(black_box(SPILLED_COUNT / 2));
    v
}

#[library_benchmark]
fn bench_smallvec_clear_inline() -> SmallVec<u64, INLINE_CAP> {
    let mut v = smallvec_with(INLINE_CAP);
    black_box(&mut v).clear();
    v
}

#[library_benchmark]
fn bench_smallvec_clear_spilled() -> SmallVec<u64, INLINE_CAP> {
    let mut v = smallvec_with(SPILLED_COUNT);
    black_box(&mut v).clear();
    v
}

#[library_benchmark]
fn bench_vec_clear() -> Vec<u64> {
    let mut v = vec_with(SPILLED_COUNT);
    black_box(&mut v).clear();
    v
}

// =========================================================================
// SHRINK TO FIT
// =========================================================================

// Shrinks far enough that the items move back inline.
#[library_benchmark]
fn bench_smallvec_shrink_to_fit_back_inline() -> SmallVec<u64, INLINE_CAP> {
    let mut v = smallvec_with(SPILLED_COUNT);
    v.truncate(black_box(INLINE_CAP / 2));
    v.shrink_to_fit();
    v
}

#[library_benchmark]
fn bench_smallvec_shrink_to_fit_spilled() -> SmallVec<u64, INLINE_CAP> {
    let mut v = smallvec_with(SPILLED_COUNT);
    v.truncate(black_box(SPILLED_COUNT / 2));
    v.shrink_to_fit();
    v
}

#[library_benchmark]
fn bench_vec_shrink_to_fit() -> Vec<u64> {
    let mut v = vec_with(SPILLED_COUNT);
    v.truncate(black_box(SPILLED_COUNT / 2));
    v.shrink_to_fit();
    v
}

// =========================================================================
// DEDUP
// =========================================================================

// Every item appears twice in a row, so half of them are removed.
#[library_benchmark]
fn bench_smallvec_dedup_inline() -> SmallVec<u64, INLINE_CAP> {
    let mut v: SmallVec<u64, INLINE_CAP> =
        (0..INLINE_CAP as u64).map(|i| black_box(i / 2)).collect();
    v.dedup();
    v
}

#[library_benchmark]
fn bench_smallvec_dedup_spilled() -> SmallVec<u64, INLINE_CAP> {
    let mut v: SmallVec<u64, INLINE_CAP> = (0..SPILLED_COUNT as u64)
        .map(|i| black_box(i / 2))
        .collect();
    v.dedup();
    v
}

#[library_benchmark]
fn bench_vec_dedup() -> Vec<u64> {
    let mut v: Vec<u64> = (0..SPILLED_COUNT as u64)
        .map(|i| black_box(i / 2))
        .collect();
    v.dedup();
    v
}

// =========================================================================
// SPLICE
// =========================================================================

// Replaces 2 items with 4, staying within the inline capacity.
#[library_benchmark]
fn bench_smallvec_splice_inline() -> SmallVec<u64, INLINE_CAP> {
    let mut v = smallvec_with(INLINE_CAP / 2);
    drop(v.splice(black_box(2..4), black_box([7u64; 4])));
    v
}

#[library_benchmark]
fn bench_smallvec_splice_spilled() -> SmallVec<u64, INLINE_CAP> {
    let mut v = smallvec_with(SPILLED_COUNT);
    drop(v.splice(black_box(10..20), black_box([7u64; 20])));
    v
}

#[library_benchmark]
fn bench_vec_splice() -> Vec<u64> {
    let mut v = vec_with(SPILLED_COUNT);
    drop(v.splice(black_box(10..20), black_box([7u64; 20])));
    v
}

// =========================================================================
// GROUPS & MAIN
// =========================================================================

library_benchmark_group!(
    name = push_group;
    benchmarks =
        bench_smallvec_push_inline,
        bench_smallvec_push_spilled,
        bench_vec_push,
        bench_vec_push_with_spilled_capacity,
);

library_benchmark_group!(
    name = pop_group;
    benchmarks =
        bench_smallvec_pop_inline,
        bench_smallvec_pop_spilled,
        bench_vec_pop,
);

library_benchmark_group!(
    name = insert_remove_group;
    benchmarks =
        bench_smallvec_insert_remove_inline,
        bench_smallvec_insert_remove_spilled,
        bench_vec_insert_remove,
);

library_benchmark_group!(
    name = slice_group;
    benchmarks =
        bench_smallvec_from_slice_inline,
        bench_smallvec_from_slice_spilled,
        bench_smallvec_extend_from_slice_inline,
        bench_smallvec_extend_from_slice_spilled,
        bench_smallvec_extend_ref_inline,
        bench_smallvec_extend_ref_spilled,
        bench_smallvec_insert_from_slice_inline,
        bench_smallvec_insert_from_slice_spilled,
        bench_vec_from_slice,
        bench_vec_extend_from_slice,
);

library_benchmark_group!(
    name = array_group;
    benchmarks =
        bench_smallvec_from_array_inline,
        bench_smallvec_from_array_spilled,
        bench_vec_from_array,
        bench_smallvec_from_elem_spilled,
);

library_benchmark_group!(
    name = iteration_group;
    benchmarks =
        bench_smallvec_into_iter_inline,
        bench_smallvec_into_iter_spilled,
        bench_smallvec_drain_inline,
        bench_smallvec_drain_spilled,
        bench_smallvec_retain_mut_inline,
        bench_smallvec_retain_mut_spilled,
        bench_vec_into_iter,
        bench_vec_drain,
        bench_vec_retain_mut,
);

library_benchmark_group!(
    name = clone_group;
    benchmarks =
        bench_smallvec_clone_inline,
        bench_smallvec_clone_spilled,
        bench_vec_clone,
);

library_benchmark_group!(
    name = truncate_clear_group;
    benchmarks =
        bench_smallvec_truncate_inline,
        bench_smallvec_truncate_spilled,
        bench_vec_truncate,
        bench_smallvec_clear_inline,
        bench_smallvec_clear_spilled,
        bench_vec_clear,
);

library_benchmark_group!(
    name = shrink_to_fit_group;
    benchmarks =
        bench_smallvec_shrink_to_fit_back_inline,
        bench_smallvec_shrink_to_fit_spilled,
        bench_vec_shrink_to_fit,
);

library_benchmark_group!(
    name = dedup_group;
    benchmarks =
        bench_smallvec_dedup_inline,
        bench_smallvec_dedup_spilled,
        bench_vec_dedup,
);

library_benchmark_group!(
    name = splice_group;
    benchmarks =
        bench_smallvec_splice_inline,
        bench_smallvec_splice_spilled,
        bench_vec_splice,
);

main!(
    library_benchmark_groups = push_group,
    pop_group,
    insert_remove_group,
    slice_group,
    array_group,
    iteration_group,
    clone_group,
    truncate_clear_group,
    shrink_to_fit_group,
    dedup_group,
    splice_group
);
