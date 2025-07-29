use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use orx_closure::*;

type Weight = i32;
type Jagged = Vec<Vec<Weight>>;

// data
#[allow(dead_code)]
fn get_jagged(n: usize) -> Jagged {
    let mut jagged = Jagged::with_capacity(n);
    for i in 0..n {
        jagged.push(Vec::with_capacity(n));
        for j in 0..n {
            jagged[i].push((i + j) as Weight)
        }
    }
    jagged
}

// variants
struct HoldingFn<F: Fn((usize, usize)) -> Weight> {
    fun: F,
}
impl<F: Fn((usize, usize)) -> Weight> HoldingFn<F> {
    #[inline(always)]
    fn get(&self, ij: (usize, usize)) -> Weight {
        (self.fun)(ij)
    }
}

struct HoldingBoxDynFn {
    fun: Box<dyn Fn((usize, usize)) -> Weight>,
}
impl HoldingBoxDynFn {
    #[inline(always)]
    fn get(&self, ij: (usize, usize)) -> Weight {
        (self.fun)(ij)
    }
}

// benchmark computations
fn closure(n: usize, fun: &Closure<Vec<Vec<Weight>>, (usize, usize), Weight>) -> Weight {
    let fun = fun.as_fn();
    let mut sum = 0;
    for i in 0..n {
        for j in 0..n {
            let value = fun((i, j));
            sum += value;
        }
    }
    sum
}
fn holding_box_dyn_fn(n: usize, fun: &HoldingBoxDynFn) -> Weight {
    let mut sum = 0;
    for i in 0..n {
        for j in 0..n {
            let value = fun.get((i, j));
            sum += value;
        }
    }
    sum
}
fn holding_fn<F: Fn((usize, usize)) -> Weight>(n: usize, fun: &HoldingFn<F>) -> Weight {
    let mut sum = 0;
    for i in 0..n {
        for j in 0..n {
            let value = fun.get((i, j));
            sum += value;
        }
    }
    sum
}
fn impl_fn<F: Fn((usize, usize)) -> Weight>(n: usize, fun: F) -> Weight {
    let mut sum = 0;
    for i in 0..n {
        for j in 0..n {
            let value = fun((i, j));
            sum += value;
        }
    }
    sum
}

fn fun_as_a_field(c: &mut Criterion) {
    let treatments = [10_000];

    let mut group = c.benchmark_group("FunAsAField");

    for n in treatments {
        group.bench_with_input(BenchmarkId::new("holding_fn", n), &n, |b, &n| {
            let data = get_jagged(n);
            let fun = HoldingFn {
                fun: move |(i, j): (usize, usize)| data[i][j],
            };
            b.iter(|| holding_fn(n, &fun))
        });

        group.bench_with_input(BenchmarkId::new("closure", n), &n, |b, &n| {
            let data = get_jagged(n);
            let fun = Capture(data).fun(|jagged, (i, j): (usize, usize)| jagged[i][j]);
            b.iter(|| closure(n, &fun))
        });

        group.bench_with_input(BenchmarkId::new("closure_as_fn", n), &n, |b, &n| {
            let data = get_jagged(n);
            let fun = Capture(data).fun(|jagged, (i, j): (usize, usize)| jagged[i][j]);
            b.iter(|| impl_fn(n, fun.as_fn()))
        });

        group.bench_with_input(BenchmarkId::new("holding_box_dyn_fn", n), &n, |b, &n| {
            let data = get_jagged(n);
            let fun = HoldingBoxDynFn {
                fun: Box::new(move |(i, j): (usize, usize)| data[i][j]),
            };
            b.iter(|| holding_box_dyn_fn(n, &fun))
        });
    }

    group.finish();
}

criterion_group!(benches, fun_as_a_field);
criterion_main!(benches);
