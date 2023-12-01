use orx_closure::*;
use std::collections::{hash_map::RandomState, HashMap};

#[test]
fn test_fun() {
    let fun = Capture(vec!["john".to_string(), "doe".to_string()]).fun(|x, i: usize| x[i].clone());
    validate_fun(fun);

    let fun = Capture(["john".to_string(), "doe".to_string()]).fun(|x, i: usize| x[i].clone());
    validate_fun(fun);

    let map: HashMap<usize, String, RandomState> =
        HashMap::from_iter([(0usize, "john".to_string()), (1, "doe".to_string())].into_iter());
    let fun = Capture(map).fun(|x, i: usize| x.get(&i).unwrap().clone());
    validate_fun(fun);
}
fn validate_fun<F: Fun<usize, String>>(fun: F) {
    assert_eq!(String::from("john"), fun.call(0));
    assert_eq!(String::from("doe"), fun.call(1));
}

#[test]
fn test_funref() {
    let fun =
        Capture(vec!["john".to_string(), "doe".to_string()]).fun_ref(|x, i: usize| x[i].as_str());
    validate_fun_ref(fun);

    let fun = Capture(["john".to_string(), "doe".to_string()]).fun_ref(|x, i: usize| x[i].as_str());
    validate_fun_ref(fun);

    let map: HashMap<usize, String, RandomState> =
        HashMap::from_iter([(0usize, "john".to_string()), (1, "doe".to_string())].into_iter());
    let fun = Capture(map).fun_ref(|x, i: usize| x.get(&i).unwrap().as_str());
    validate_fun_ref(fun);
}
fn validate_fun_ref<F: FunRef<usize, str>>(fun: F) {
    assert_eq!("john", fun.call(0));
    assert_eq!("doe", fun.call(1));
}

#[test]
fn test_funoptref() {
    let fun = Capture(vec!["john".to_string(), "doe".to_string()])
        .fun_option_ref(|x, i: usize| x.get(i).map(|x| x.as_str()));
    validate_fun_opt_ref(fun);

    let fun = Capture(["john".to_string(), "doe".to_string()])
        .fun_option_ref(|x, i: usize| x.get(i).map(|x| x.as_str()));
    validate_fun_opt_ref(fun);

    let map: HashMap<usize, String, RandomState> =
        HashMap::from_iter([(0usize, "john".to_string()), (1, "doe".to_string())].into_iter());
    let fun = Capture(map).fun_option_ref(|x, i: usize| x.get(&i).map(|x| x.as_str()));
    validate_fun_opt_ref(fun);
}
fn validate_fun_opt_ref<F: FunOptRef<usize, str>>(fun: F) {
    assert_eq!(Some("john"), fun.call(0));
    assert_eq!(Some("doe"), fun.call(1));
    assert_eq!(None, fun.call(2));
}

#[test]
fn test_funresref() {
    let fun = Capture(vec!["john".to_string(), "doe".to_string()])
        .fun_result_ref(|x, i: usize| x.get(i).map(|x| x.as_str()).ok_or(42));
    validate_fun_res_ref(fun);

    let fun = Capture(["john".to_string(), "doe".to_string()])
        .fun_result_ref(|x, i: usize| x.get(i).map(|x| x.as_str()).ok_or(42));
    validate_fun_res_ref(fun);

    let map: HashMap<usize, String, RandomState> =
        HashMap::from_iter([(0usize, "john".to_string()), (1, "doe".to_string())].into_iter());
    let fun = Capture(map).fun_result_ref(|x, i: usize| x.get(&i).map(|x| x.as_str()).ok_or(42));
    validate_fun_res_ref(fun);
}
fn validate_fun_res_ref<F: FunResRef<usize, str, usize>>(fun: F) {
    assert_eq!(Ok("john"), fun.call(0));
    assert_eq!(Ok("doe"), fun.call(1));
    assert_eq!(Err(42), fun.call(2));
}
