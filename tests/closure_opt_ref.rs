use orx_closure::*;
use std::collections::HashMap;

#[test]
fn owning_higher_order_function() {
    fn make_owning_function(data: Vec<i32>) -> ClosureOptRef<Vec<i32>, usize, i32> {
        Capture(data).fun_option_ref(|data: &Vec<i32>, i| data.get(i))
    }

    let data = vec![0, 1, 2, 3, 4];

    let closure = make_owning_function(data);

    assert_eq!(Some(&0), closure.call(0));
    assert_eq!(Some(&3), closure.call(3));
    assert_eq!(None, closure.call(13));
    assert_higher_order_function(closure.as_fn());

    let data = closure.into_captured_data();
    assert_eq!(5, data.len());
}
#[test]
fn referencing_higher_order_function() {
    fn make_owning_function(data: &Vec<i32>) -> ClosureOptRef<&Vec<i32>, usize, i32> {
        Capture(data).fun_option_ref(|data: &&Vec<i32>, i| data.get(i))
    }

    let data = vec![0, 1, 2, 3, 4];

    let closure = make_owning_function(&data);

    assert_higher_order_function(closure.as_fn());

    let data = closure.into_captured_data();
    assert_eq!(5, data.len());
}
fn assert_higher_order_function<'a, F: Fn(usize) -> Option<&'a i32>>(fun: F) {
    assert_eq!(Some(&0), fun(0));
    assert_eq!(Some(&3), fun(3));
    assert_eq!(None, fun(13));
}

#[test]
fn owning_field() {
    struct People<'a> {
        get_age: ClosureOptRef<HashMap<String, u32>, &'a str, u32>,
    }
    impl<'a> People<'a> {
        fn age_of(&self, empires: &'a str) -> Option<&u32> {
            self.get_age.call(empires)
        }
    }

    let map = HashMap::from_iter([(String::from("john"), 42), (String::from("doe"), 33)]);
    let people = People {
        get_age: Capture(map).fun_option_ref(|m, p| m.get(p)),
    };

    assert_eq!(Some(&42), people.age_of("john"));
    //assert_eq!(2, map.len()); // map is moved into the closure, this won't compile
    assert_eq!(None, people.age_of("foo"));

    let map_back = people.get_age.into_captured_data();
    assert_eq!(2, map_back.len()); // map is moved out of the closure
}

#[test]
fn referencing_field() {
    struct People<'a> {
        get_age: ClosureOptRef<&'a HashMap<String, u32>, &'a str, u32>,
    }
    impl<'a> People<'a> {
        fn age_of(&self, empires: &'a str) -> Option<&u32> {
            self.get_age.call(empires)
        }
    }

    let map = HashMap::from_iter([(String::from("john"), 42), (String::from("doe"), 33)]);
    let people = People {
        get_age: Capture(&map).fun_option_ref(|m, p| m.get(p)),
    };

    assert_eq!(Some(&42), people.age_of("john"));
    assert_eq!(2, map.len()); // map is only referenced by the closure
    assert_eq!(None, people.age_of("foo"));
}
