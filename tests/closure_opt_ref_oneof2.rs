use orx_closure::*;
use std::collections::HashMap;

type Edge = (usize, usize);
type Weight = i32;
type Jagged = Vec<Vec<Option<Weight>>>;
type Map = Vec<HashMap<usize, Weight>>;

pub struct WeightsProvider {
    fun: ClosureOptRefOneOf2<Jagged, Map, Edge, i32>,
}
impl WeightsProvider {
    fn weight(&self, i: usize, j: usize) -> Option<&Weight> {
        self.fun.call((i, j))
    }
}

/* edge weights
    from    to  weight
    0       0   0
    0       1   4
    0       2   2
    1       0   None
    1       1   0
    1       2   5
    2       0   None
    2       1   None
    2       2   0
*/

#[test]
fn jagged() {
    let weights = vec![
        vec![Some(0), Some(4), Some(2)],
        vec![None, Some(0), Some(5)],
        vec![None, None, Some(0)],
    ];
    let closure =
        Capture(weights).fun_option_ref(|jagged, edge: Edge| jagged[edge.0][edge.1].as_ref());

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
    let closure = Capture(weights).fun_option_ref(|map, edge: Edge| {
        if edge.0 == edge.1 {
            Some(&0)
        } else {
            map[edge.0].get(&edge.1)
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
    assert_eq!(Some(&0), provider.weight(0, 0));
    assert_eq!(Some(&4), provider.weight(0, 1));
    assert_eq!(Some(&2), provider.weight(0, 2));

    assert_eq!(None, provider.weight(1, 0));
    assert_eq!(Some(&0), provider.weight(1, 1));
    assert_eq!(Some(&5), provider.weight(1, 2));

    assert_eq!(None, provider.weight(2, 0));
    assert_eq!(None, provider.weight(2, 1));
    assert_eq!(Some(&0), provider.weight(2, 2));
}
fn assert_provider_as_closure(closure: &ClosureOptRefOneOf2<Jagged, Map, Edge, i32>) {
    assert_eq!(Some(&0), closure.call((0, 0)));
    assert_eq!(Some(&4), closure.call((0, 1)));
    assert_eq!(Some(&2), closure.call((0, 2)));

    assert_eq!(None, closure.call((1, 0)));
    assert_eq!(Some(&0), closure.call((1, 1)));
    assert_eq!(Some(&5), closure.call((1, 2)));

    assert_eq!(None, closure.call((2, 0)));
    assert_eq!(None, closure.call((2, 1)));
    assert_eq!(Some(&0), closure.call((2, 2)));
}
fn assert_provider_as_fn<'a, F: Fn(Edge) -> Option<&'a i32>>(fun: F) {
    assert_eq!(Some(&0), fun((0, 0)));
    assert_eq!(Some(&4), fun((0, 1)));
    assert_eq!(Some(&2), fun((0, 2)));

    assert_eq!(None, fun((1, 0)));
    assert_eq!(Some(&0), fun((1, 1)));
    assert_eq!(Some(&5), fun((1, 2)));

    assert_eq!(None, fun((2, 0)));
    assert_eq!(None, fun((2, 1)));
    assert_eq!(Some(&0), fun((2, 2)));
}
