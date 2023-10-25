use criterion::{criterion_group, criterion_main, Bencher, Criterion};

use freeout::entities::core::freeout::{Freeout, FreeoutOptions};
use freeout::readers::markdown::MarkdownReader;

fn standard_markdown_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("outline depth scaling");

    for depths in [100, 200, 400, 800, 1600].iter() {
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(depths),
            depths,
            |b, depths| {
                let mut markdown_with_increasing_depths = "".to_string();

                for i in 0..*depths {
                    for j in 0..5 {
                        let marker = "#".repeat(j);
                        markdown_with_increasing_depths
                            .push_str(&format!("{} Title {}\n", marker, i));
                    }
                }

                generate_outline(
                    b,
                    markdown_with_increasing_depths,
                    FreeoutOptions {
                        include_content: true,
                    },
                );
            },
        );
    }
    group.finish();
}

fn generate_outline(
    b: &mut Bencher,
    mut markdown_with_increasing_depths: String,
    options: FreeoutOptions,
) {
    let reader = MarkdownReader::default();
    b.iter(|| {
        let _outline = Freeout::new(
            markdown_with_increasing_depths.clone(),
            Some(options.clone()),
        )
        .outline(&reader)
        .unwrap();
    })
}

criterion_group!(benches, standard_markdown_benchmark);
criterion_main!(benches);
