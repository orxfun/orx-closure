use orx_closure::*;
use std::collections::HashMap;

#[test]
fn owning_higher_order_function() {
    fn make_owning_function(data: Vec<i32>) -> Closure<Vec<i32>, usize, i32> {
        let dbl = |data: &Vec<i32>, i| data[i] * 2;
        Capture(data).fun(dbl)
    }

    let data = vec![0, 1, 2, 3, 4];

    let closure = make_owning_function(data);

    assert_eq!(0, closure.call(0));
    assert_eq!(6, closure.call(3));

    let data = closure.into_captured_data();
    assert_eq!(5, data.len());
}

#[test]
fn referencing_higher_order_function() {
    fn make_referencing_function(data: &Vec<i32>) -> Closure<&Vec<i32>, usize, i32> {
        Capture(data).fun(|data, i| data[i] * 2)
    }

    let data = vec![0, 1, 2, 3, 4];

    let closure = make_referencing_function(&data);

    assert_eq!(0, closure.call(0));
    assert_eq!(6, closure.call(3));
}

#[test]
fn owning_field() {
    struct People<'a> {
        get_age: Closure<HashMap<String, u32>, &'a str, u32>,
    }
    impl<'a> People<'a> {
        fn age_of(&self, empires: &'a str) -> u32 {
            self.get_age.call(empires)
        }
    }

    let map =
        HashMap::from_iter([(String::from("john"), 42), (String::from("doe"), 33)].into_iter());
    let people = People {
        get_age: Capture(map).fun(|m, p| *m.get(p).unwrap_or(&0)),
    };

    assert_eq!(42, people.age_of("john"));
    //assert_eq!(2, map.len()); // map is moved into the closure, this won't compile
    assert_eq!(0, people.age_of("foo"));

    let map_back = people.get_age.into_captured_data();
    assert_eq!(2, map_back.len()); // map is moved out of the closure
}

#[test]
fn referencing_field() {
    struct People<'a> {
        get_age: Closure<&'a HashMap<String, u32>, &'a str, u32>,
    }
    impl<'a> People<'a> {
        fn age_of(&self, empires: &'a str) -> u32 {
            self.get_age.call(empires)
        }
    }

    let map =
        HashMap::from_iter([(String::from("john"), 42), (String::from("doe"), 33)].into_iter());
    let people = People {
        get_age: Capture(&map).fun(|m, p| *m.get(p).unwrap_or(&0)),
    };

    assert_eq!(42, people.age_of("john"));
    assert_eq!(2, map.len()); // map is only referenced by the closure
    assert_eq!(0, people.age_of("foo"));
}
