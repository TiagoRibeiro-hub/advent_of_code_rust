use criterion::{black_box, criterion_group, criterion_main, Criterion};
use adv_code_2015::{ex1::*, ex2::*, ex3::*};

// fn floor_bench(c: &mut Criterion) {
//     let input = black_box(read_input_one());

//     c.bench_function("floor_chars", |b| b.iter(|| floor_chars(&input)));
//     c.bench_function("floor_bytes", |b| b.iter(|| floor_bytes(&input)));
// }

// fn floor_basement_bench(c: &mut Criterion) {
//     let input = black_box(read_input_one());
//     c.bench_function("floor_basement_same_var", |b| {
//         b.iter(|| floor_basement_same_var(&input))
//     });
    
//     c.bench_function("floor_basement", |b| b.iter(|| floor_basement(&input)));
    
// }

// fn wrapper_paper_bench(c: &mut Criterion) {
//     c.bench_function("wrapper_paper_read_input", |b| b.iter(|| wrapper_paper_read_input()));
    
//     c.bench_function("wrapper_paper_read_lines", |b| b.iter(|| wrapper_paper_read_lines()));
    
//     c.bench_function("wrapper_paper_read_input_area", |b| b.iter(|| wrapper_paper_read_input_area()));
    
//     c.bench_function("wrapper_paper_read_lines_area", |b| b.iter(|| wrapper_paper_read_lines_area()));
// }

// fn ribbon_length_bench(c: &mut Criterion) {
//     c.bench_function("ribbon_length", |b| {
//         b.iter(|| ribbon_length())
//     });

//     c.bench_function("ribbon_length_2", |b| {
//         b.iter(|| ribbon_length_2())
//     });
// }

pub fn delivers_presents_bench(c: &mut Criterion) {
    let input = black_box(read_input_three());
    c.bench_function("delivers_presents", |b| {
        b.iter(|| delivers_presents(&input))
    });

    // c.bench_function("delivers_presents_hash_set", |b| {
    //     b.iter(|| delivers_presents_hash_set(&input))
    // });

    c.bench_function("delivers_presents_robot", |b| {
        b.iter(|| delivers_presents_robot(&input))
    });
}

criterion_group!(
    benches,
    // floor_bench,
    // floor_basement_bench,
    // wrapper_paper_bench,
    // ribbon_length_bench,
    delivers_presents_bench
);
criterion_main!(benches);
