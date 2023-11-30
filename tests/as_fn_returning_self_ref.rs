use orx_closure::*;
use std::{collections::HashMap, rc::Rc};

fn test_fun<'a, F: Fn(char) -> &'a usize>(fun: F) {
    assert_eq!(fun('a'), &1);
    assert_eq!(fun('b'), &2);
    assert_eq!(fun('c'), &42);
}

#[test]
fn nocapture() {
    let fun = Capture(()).fun(|_: &(), c: char| match c {
        'a' => &1,
        'b' => &2,
        _ => &42,
    });
    test_fun(fun.as_fn());
    test_fun(|x| fun.call(x));
}

#[test]
fn capture_data() {
    let map = HashMap::<char, usize>::from_iter([('a', 1), ('b', 2)].into_iter());
    let fun = Capture(map).fun_ref(|map, c| map.get(&c).unwrap_or(&42));
    test_fun(fun.as_fn());
    test_fun(|x| fun.call(x));
}

#[test]
fn capture_ref() {
    let map = HashMap::<char, usize>::from_iter([('a', 1), ('b', 2)].into_iter());
    let fun = Capture(&map).fun(|map, c| map.get(&c).unwrap_or(&42));
    test_fun(fun.as_fn());
    test_fun(|x| fun.call(x));
}

#[test]
fn capture_deref() {
    let map = HashMap::<char, usize>::from_iter([('a', 1), ('b', 2)].into_iter());
    let map = Rc::new(map);
    let fun = Capture(map.clone()).fun_ref(|map, c| map.get(&c).unwrap_or(&42));
    test_fun(fun.as_fn());
    test_fun(|x| fun.call(x));

    let map = HashMap::<char, usize>::from_iter([('a', 1), ('b', 2)].into_iter());
    let map = Box::new(map);
    let fun = Capture(map).fun_ref(|map, c| map.get(&c).unwrap_or(&42));
    test_fun(fun.as_fn());
    test_fun(|x| fun.call(x));
}
