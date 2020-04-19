#[macro_use]
extern crate bencher;
extern crate rand;
extern crate adler32;

use bencher::Bencher;
use rand::{thread_rng, Rng};
use adler32::RollingAdler32;

fn bench(b: &mut Bencher, size: usize, adler: &mut RollingAdler32) {
    let mut in_bytes = vec![0u8; size];
    thread_rng().fill_bytes(&mut in_bytes);

    b.iter(|| {
        adler.update_buffer(&in_bytes);
        bencher::black_box(adler.hash())
    });
    b.bytes = size as u64;
}

fn bench_512b(b: &mut Bencher) {
    bench(b, 512, &mut RollingAdler32::new())
}

fn bench_100kib(b: &mut Bencher) {
    bench(b, 1024 * 100, &mut RollingAdler32::new())
}

benchmark_group!(bench_default, bench_512b, bench_100kib);

benchmark_main!(bench_default);
