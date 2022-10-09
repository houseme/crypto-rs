use bencher::{benchmark_group, benchmark_main, Bencher};
use crypto_rs::{md5, rsa};

fn bench_new(b: &mut Bencher) {
    b.iter(|| rsa::new());
}
