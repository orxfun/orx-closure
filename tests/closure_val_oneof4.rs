use orx_closure::{Capture, ClosureOneOf4};
use std::collections::HashMap;

type Edge = (usize, usize);
type Weight = i32;
type Jagged = Vec<Vec<Weight>>;
type Flat = (usize, Vec<Weight>);
type AlwaysOne = ();
type Map = Vec<HashMap<usize, Weight>>;
const INF: Weight = Weight::MAX;

pub struct WeightsProvider {
    fun: ClosureOneOf4<Jagged, Map, Flat, AlwaysOne, Edge, i32>,
}
impl WeightsProvider {
    fn weight(&self, i: usize, j: usize) -> Weight {
        self.fun.call((i, j))
    }
}

/* edge weights
    from    to  weight
    0       0   0
    0       1   4
    0       2   2
    1       0   inf
    1       1   0
    1       2   5
    2       0   inf
    2       1   inf
    2       2   0
*/

#[test]
fn jagged() {
    let weights = vec![vec![0, 4, 2], vec![INF, 0, 5], vec![INF, INF, 0]];
    let closure = Capture(weights).fun(|jagged, edge: Edge| jagged[edge.0][edge.1]);

    let provider = WeightsProvider {
        fun: closure.into_oneof4_var1(),
    };

    assert_provider(&provider);
    assert_provider_as_closure(&provider.fun);
    assert_provider_as_fn(provider.fun.as_fn());
}

#[test]
fn map() {
    let weights = vec![
        HashMap::from_iter([(1, 4), (2, 2)].into_iter()),
        HashMap::from_iter([(2, 5)].into_iter()),
        HashMap::new(),
    ];
    let closure = Capture(weights).fun(|map, edge: Edge| {
        if edge.0 == edge.1 {
            0
        } else {
            map[edge.0].get(&edge.1).cloned().unwrap_or(INF)
        }
    });

    let provider = WeightsProvider {
        fun: closure.into_oneof4_var2(),
    };

    assert_provider(&provider);
    assert_provider_as_closure(&provider.fun);
    assert_provider_as_fn(provider.fun.as_fn());
}

#[test]
fn flat() {
    let weights = (3, vec![0, 4, 2, INF, 0, 5, INF, INF, 0]);
    let closure = Capture(weights).fun(|flat, edge: Edge| {
        let n = flat.0;
        let idx = n * edge.0 + edge.1;
        flat.1[idx]
    });

    let provider = WeightsProvider {
        fun: closure.into_oneof4_var3(),
    };

    assert_provider(&provider);
    assert_provider_as_closure(&provider.fun);
    assert_provider_as_fn(provider.fun.as_fn());
}

#[test]
fn always_one() {
    let closure = Capture(()).fun(|_, _: Edge| 1);

    let provider = WeightsProvider {
        fun: closure.into_oneof4_var4(),
    };

    for i in 0..3 {
        for j in 0..3 {
            assert_eq!(1, provider.weight(i, j));
        }
    }
}

// validators
fn assert_provider(provider: &WeightsProvider) {
    assert_eq!(0, provider.weight(0, 0));
    assert_eq!(4, provider.weight(0, 1));
    assert_eq!(2, provider.weight(0, 2));

    assert_eq!(INF, provider.weight(1, 0));
    assert_eq!(0, provider.weight(1, 1));
    assert_eq!(5, provider.weight(1, 2));

    assert_eq!(INF, provider.weight(2, 0));
    assert_eq!(INF, provider.weight(2, 1));
    assert_eq!(0, provider.weight(2, 2));
}
fn assert_provider_as_closure(closure: &ClosureOneOf4<Jagged, Map, Flat, AlwaysOne, Edge, i32>) {
    assert_eq!(0, closure.call((0, 0)));
    assert_eq!(4, closure.call((0, 1)));
    assert_eq!(2, closure.call((0, 2)));

    assert_eq!(INF, closure.call((1, 0)));
    assert_eq!(0, closure.call((1, 1)));
    assert_eq!(5, closure.call((1, 2)));

    assert_eq!(INF, closure.call((2, 0)));
    assert_eq!(INF, closure.call((2, 1)));
    assert_eq!(0, closure.call((2, 2)));
}
fn assert_provider_as_fn<F: Fn(Edge) -> i32>(fun: F) {
    assert_eq!(0, fun((0, 0)));
    assert_eq!(4, fun((0, 1)));
    assert_eq!(2, fun((0, 2)));

    assert_eq!(INF, fun((1, 0)));
    assert_eq!(0, fun((1, 1)));
    assert_eq!(5, fun((1, 2)));

    assert_eq!(INF, fun((2, 0)));
    assert_eq!(INF, fun((2, 1)));
    assert_eq!(0, fun((2, 2)));
}
