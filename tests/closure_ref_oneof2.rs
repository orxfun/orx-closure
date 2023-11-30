use orx_closure::*;
use std::collections::HashMap;

type Edge = (usize, usize);
type Weight = i32;
type Jagged = Vec<Vec<Weight>>;
type Map = Vec<HashMap<usize, Weight>>;
const INF: Weight = Weight::MAX;

pub struct WeightsProvider {
    fun: ClosureRefOneOf2<Jagged, Map, Edge, i32>,
}
impl WeightsProvider {
    fn weight(&self, i: usize, j: usize) -> &Weight {
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
    let closure = Capture(weights).fun_ref(|jagged, edge: Edge| &jagged[edge.0][edge.1]);

    let provider = WeightsProvider {
        fun: closure.into_oneof2_var1(),
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
    let closure = Capture(weights).fun_ref(|map, edge: Edge| {
        if edge.0 == edge.1 {
            &0
        } else {
            map[edge.0].get(&edge.1).unwrap_or(&INF)
        }
    });

    let provider = WeightsProvider {
        fun: closure.into_oneof2_var2(),
    };

    assert_provider(&provider);
    assert_provider_as_closure(&provider.fun);
    assert_provider_as_fn(provider.fun.as_fn());
}

// validators
fn assert_provider(provider: &WeightsProvider) {
    assert_eq!(&0, provider.weight(0, 0));
    assert_eq!(&4, provider.weight(0, 1));
    assert_eq!(&2, provider.weight(0, 2));

    assert_eq!(&INF, provider.weight(1, 0));
    assert_eq!(&0, provider.weight(1, 1));
    assert_eq!(&5, provider.weight(1, 2));

    assert_eq!(&INF, provider.weight(2, 0));
    assert_eq!(&INF, provider.weight(2, 1));
    assert_eq!(&0, provider.weight(2, 2));
}
fn assert_provider_as_closure(closure: &ClosureRefOneOf2<Jagged, Map, Edge, i32>) {
    assert_eq!(&0, closure.call((0, 0)));
    assert_eq!(&4, closure.call((0, 1)));
    assert_eq!(&2, closure.call((0, 2)));

    assert_eq!(&INF, closure.call((1, 0)));
    assert_eq!(&0, closure.call((1, 1)));
    assert_eq!(&5, closure.call((1, 2)));

    assert_eq!(&INF, closure.call((2, 0)));
    assert_eq!(&INF, closure.call((2, 1)));
    assert_eq!(&0, closure.call((2, 2)));
}
fn assert_provider_as_fn<'a, F: Fn(Edge) -> &'a i32>(fun: F) {
    assert_eq!(&0, fun((0, 0)));
    assert_eq!(&4, fun((0, 1)));
    assert_eq!(&2, fun((0, 2)));

    assert_eq!(&INF, fun((1, 0)));
    assert_eq!(&0, fun((1, 1)));
    assert_eq!(&5, fun((1, 2)));

    assert_eq!(&INF, fun((2, 0)));
    assert_eq!(&INF, fun((2, 1)));
    assert_eq!(&0, fun((2, 2)));
}
