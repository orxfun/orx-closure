use orx_closure::*;
use std::collections::HashMap;

#[test]
fn test_debug() {
    let map = HashMap::<char, usize>::from_iter([('a', 1), ('b', 2)]);
    let fun = Capture(&map).fun(|map, c| *map.get(&c).unwrap_or(&42));

    let dbg_map = format!("Closure {{ capture: {:?} }}", map);
    let dbg_fun = format!("{:?}", fun);

    assert_eq!(dbg_map, dbg_fun)
}
