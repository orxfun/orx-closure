use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use orx_closure::*;
use std::{collections::HashMap, fmt::Display, rc::Rc};

type Edge = (usize, usize);
type Weight = i32;
type Jagged = Vec<Vec<Weight>>;
type Flat = (usize, Vec<Weight>);
type AlwaysOne = ();
type Map = Vec<HashMap<usize, Weight>>;

type Closure1 = Closure<Jagged, Edge, i32>;
type Closure2 = ClosureOneOf2<Jagged, AlwaysOne, Edge, i32>;
type Closure4 = ClosureOneOf4<Jagged, Map, Flat, AlwaysOne, Edge, i32>;

struct Treatment(DataVariant, usize);
impl Display for Treatment {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}({})", self.0, self.1)
    }
}
impl From<(DataVariant, usize)> for Treatment {
    fn from((variant, n): (DataVariant, usize)) -> Self {
        Treatment(variant, n)
    }
}

// data
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum DataVariant {
    Jagged,
    Flat,
    Map,
    AlwaysOne,
}

fn get_jagged(n: usize) -> Jagged {
    let mut jagged = Jagged::with_capacity(n);
    for i in 0..n {
        jagged.push(Vec::with_capacity(n));
        for j in 0..n {
            jagged[i].push((i + j) as i32)
        }
    }
    jagged
}
fn get_map(n: usize) -> Map {
    let mut map = Map::with_capacity(n);
    for i in 0..n {
        map.push(HashMap::with_capacity(n / 2 + 1));
        for j in 0..n {
            let sum = (i + j) as i32;
            if sum % 7 == 0 {
                map[i].insert(j, sum);
            }
        }
    }
    map
}
fn get_flat(n: usize) -> Flat {
    let mut flat = (n, Vec::with_capacity(n * n));
    for i in 0..n {
        for j in 0..n {
            flat.1.push((i + j) as i32)
        }
    }
    flat
}

// variants
fn get_fn_trait_jagged(n: usize) -> impl Fn(usize, usize) -> i32 {
    let data = get_jagged(n);
    move |i: usize, j: usize| data[i][j]
}
fn get_fn_trait_flat(n: usize) -> impl Fn(usize, usize) -> i32 {
    let data = get_flat(n);
    move |i: usize, j: usize| data.1[i * data.0 + j]
}
fn get_fn_trait_map(n: usize) -> impl Fn(usize, usize) -> i32 {
    let data = get_map(n);
    move |i: usize, j: usize| *data[i].get(&j).unwrap_or(&42)
}
fn get_fn_trait_always_one() -> impl Fn(usize, usize) -> i32 {
    |_: usize, _: usize| 1
}
fn get_box_dyn_fn_trait(n: usize, data: DataVariant) -> Box<dyn Fn(usize, usize) -> i32> {
    match data {
        DataVariant::Jagged => {
            let data = get_jagged(n);
            Box::new(move |i: usize, j: usize| data[i][j])
        }
        DataVariant::Flat => {
            let data = get_flat(n);
            Box::new(move |i: usize, j: usize| data.1[i * data.0 + j])
        }
        DataVariant::Map => {
            let data = get_map(n);
            Box::new(move |i: usize, j: usize| *data[i].get(&j).unwrap_or(&42))
        }
        DataVariant::AlwaysOne => Box::new(|_: usize, _: usize| 1),
    }
}
fn get_rc_dyn_fn_trait(n: usize, data: DataVariant) -> Rc<dyn Fn(usize, usize) -> i32> {
    match data {
        DataVariant::Jagged => {
            let data = get_jagged(n);
            Rc::new(move |i: usize, j: usize| data[i][j])
        }
        DataVariant::Flat => {
            let data = get_flat(n);
            Rc::new(move |i: usize, j: usize| data.1[i * data.0 + j])
        }
        DataVariant::Map => {
            let data = get_map(n);
            Rc::new(move |i: usize, j: usize| *data[i].get(&j).unwrap_or(&42))
        }
        DataVariant::AlwaysOne => Rc::new(|_: usize, _: usize| 1),
    }
}
fn get_closure1(n: usize, data: DataVariant) -> Closure1 {
    match data {
        DataVariant::Jagged => {
            let data = get_jagged(n);
            Capture(data).fun(|data, ij: Edge| data[ij.0][ij.1])
        }
        _ => panic!("Closure1 handles only Jagged"),
    }
}
fn get_closure2(n: usize, data: DataVariant) -> Closure2 {
    match data {
        DataVariant::Jagged => {
            let data = get_jagged(n);
            Capture(data)
                .fun(|data, ij: Edge| data[ij.0][ij.1])
                .into_oneof2_var1()
        }
        DataVariant::AlwaysOne => Capture(()).fun(|_, _: Edge| 1).into_oneof2_var2(),
        _ => panic!("Closure2 is polymorphic over Jagged & AlwaysOne"),
    }
}
fn get_closure4(n: usize, data: DataVariant) -> Closure4 {
    match data {
        DataVariant::Jagged => {
            let data = get_jagged(n);
            Capture(data)
                .fun(|data, ij: Edge| data[ij.0][ij.1])
                .into_oneof4_var1()
        }
        DataVariant::Flat => {
            let data = get_flat(n);
            Capture(data)
                .fun(|data, ij: Edge| data.1[ij.0 * data.0 + ij.1])
                .into_oneof4_var3()
        }
        DataVariant::Map => {
            let data = get_map(n);
            Capture(data)
                .fun(|data, ij: Edge| *data[ij.0].get(&ij.1).unwrap_or(&42))
                .into_oneof4_var2()
        }
        DataVariant::AlwaysOne => Capture(()).fun(|_, _: Edge| 1).into_oneof4_var4(),
    }
}

// functions
fn fn_trait<F: Fn(usize, usize) -> i32>(n: usize, fun: &F) -> i32 {
    let mut sum = 0;
    for i in 0..n {
        for j in 0..n {
            let value = fun(i, j);
            sum += value;
        }
    }
    sum
}
fn rc_fn_trait(n: usize, fun: &Rc<dyn Fn(usize, usize) -> i32>) -> i32 {
    let mut sum = 0;
    for i in 0..n {
        for j in 0..n {
            let value = fun(i, j);
            sum += value;
        }
    }
    sum
}
fn closure<C: Fn((usize, usize)) -> i32>(n: usize, fun: &C) -> i32 {
    let mut sum = 0;
    for i in 0..n {
        for j in 0..n {
            let value = fun((i, j));
            sum += value;
        }
    }
    sum
}

fn bench_unified_access(c: &mut Criterion) {
    let n = [1_000, 5_000, 10_000];
    // let n = [1000];
    // let variant = [DataVariant::Jagged, DataVariant::AlwaysOne];
    let variant = [DataVariant::Jagged];

    let treatments: Vec<_> = n
        .into_iter()
        .flat_map(|n| {
            variant
                .into_iter()
                .map(move |variant| Treatment(variant, n))
        })
        .collect();

    let mut group = c.benchmark_group("UnifiedIndexedAccess");

    for treatment in &treatments {
        group.bench_with_input(
            BenchmarkId::new("Fn", treatment),
            treatment,
            |b, treatment| match treatment.0 {
                DataVariant::Jagged => {
                    let fun = get_fn_trait_jagged(treatment.1);
                    b.iter(|| fn_trait(treatment.1, &fun))
                }
                DataVariant::Flat => {
                    let fun = get_fn_trait_flat(treatment.1);
                    b.iter(|| fn_trait(treatment.1, &fun))
                }
                DataVariant::Map => {
                    let fun = get_fn_trait_map(treatment.1);
                    b.iter(|| fn_trait(treatment.1, &fun))
                }
                DataVariant::AlwaysOne => {
                    let fun = get_fn_trait_always_one();
                    b.iter(|| fn_trait(treatment.1, &fun))
                }
            },
        );

        group.bench_with_input(
            BenchmarkId::new("BoxDynFn", treatment),
            treatment,
            |b, treatment| {
                let fun = get_box_dyn_fn_trait(treatment.1, treatment.0);
                b.iter(|| fn_trait(treatment.1, &fun))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("RcDynFn", treatment),
            treatment,
            |b, treatment| {
                let fun = get_rc_dyn_fn_trait(treatment.1, treatment.0);
                b.iter(|| rc_fn_trait(treatment.1, &fun))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Closure1", treatment),
            treatment,
            |b, treatment| {
                let fun = get_closure1(treatment.1, treatment.0);
                b.iter(|| closure(treatment.1, &fun.as_fn()))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Closure2", treatment),
            treatment,
            |b, treatment| {
                let fun = get_closure2(treatment.1, treatment.0);
                b.iter(|| closure(treatment.1, &fun.as_fn()))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Closure4", treatment),
            treatment,
            |b, treatment| {
                let fun = get_closure4(treatment.1, treatment.0);
                b.iter(|| closure(treatment.1, &fun.as_fn()))
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_unified_access);
criterion_main!(benches);
