use criterion::{black_box, criterion_group, criterion_main, Criterion};

use steam_shortcuts_util::parse_shortcuts;

fn parser_benchmark(c: &mut Criterion) {
    let content_vec = std::fs::read("src/testdata/shortcuts.vdf").unwrap();
    let content = content_vec.as_slice();

    c.bench_function("file parser", |b| {
        b.iter(|| {
            let shortcuts = parse_shortcuts(content).unwrap();
            black_box(shortcuts);
        })
    });
}

criterion_group!(benches, parser_benchmark);
criterion_main!(benches);
