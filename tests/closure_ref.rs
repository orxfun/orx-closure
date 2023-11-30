use orx_closure::*;
use std::collections::HashMap;

#[test]
fn ex() {
    use orx_closure::*;

    type Toy = String;
    type MyErr = &'static str;
    struct Cat {
        name: String,
        favorite_toys: Vec<Toy>,
    }
    struct Dog {
        name: String,
        nickname: String,
        favorite_toys: Vec<Toy>,
    }

    struct PresentIdeas<'a> {
        // for cats or dogs
        for_pet: ClosureResRefOneOf2<Vec<Cat>, Vec<Dog>, &'a str, [Toy], MyErr>,
    }

    // cats
    let cats = vec![Cat {
        name: "bella".to_string(),
        favorite_toys: vec!["ball".to_string()],
    }];
    let present_ideas = PresentIdeas {
        for_pet: Capture(cats)
            .fun_result_ref(|cats, name| {
                cats.iter()
                    .find(|cat| cat.name == name)
                    .map(|cat| cat.favorite_toys.as_slice())
                    .ok_or("pet name is absent")
            })
            .into_oneof2_var1(),
    };

    assert_eq!(
        Ok(vec!["ball".to_string()].as_slice()),
        present_ideas.for_pet.call("bella")
    );
    assert_eq!(
        Err("pet name is absent"),
        present_ideas.for_pet.call("luna")
    );

    // dogs
    let dogs = vec![Dog {
        name: "luke".to_string(),
        nickname: "dogzilla".to_string(),
        favorite_toys: vec!["toy turtle".to_string()],
    }];
    let present_ideas = PresentIdeas {
        for_pet: Capture(dogs)
            .fun_result_ref(|dogs, name| {
                dogs.iter()
                    .find(|dog| dog.name == name || dog.nickname == name)
                    .map(|dog| dog.favorite_toys.as_slice())
                    .ok_or("pet name is absent")
            })
            .into_oneof2_var2(),
    };
    assert_eq!(
        Ok(vec!["toy turtle".to_string()].as_slice()),
        present_ideas.for_pet.call("luke")
    );
    assert_eq!(
        Ok(vec!["toy turtle".to_string()].as_slice()),
        present_ideas.for_pet.call("dogzilla")
    );
    assert_eq!(Err("pet name is absent"), present_ideas.for_pet.call("tux"));
}

#[test]
fn owning_higher_order_function() {
    fn make_owning_function(data: Vec<i32>) -> ClosureRef<Vec<i32>, usize, i32> {
        Capture(data).fun_ref(|data: &Vec<i32>, i| &data[i])
    }

    let data = vec![0, 1, 2, 3, 4];

    let closure = make_owning_function(data);

    assert_eq!(&0, closure.call(0));
    assert_eq!(&3, closure.call(3));

    let data = closure.into_captured_data();
    assert_eq!(5, data.len());
}

#[test]
fn referencing_higher_order_function() {
    fn make_owning_function(data: &Vec<i32>) -> ClosureRef<&Vec<i32>, usize, i32> {
        Capture(data).fun_ref(|data: &&Vec<i32>, i| &data[i])
    }

    let data = vec![0, 1, 2, 3, 4];

    let closure = make_owning_function(&data);

    assert_eq!(&0, closure.call(0));
    assert_eq!(&3, closure.call(3));

    let data = closure.into_captured_data();
    assert_eq!(5, data.len());
}

#[test]
fn owning_field() {
    struct People<'a> {
        get_age: ClosureRef<HashMap<String, u32>, &'a str, u32>,
    }
    impl<'a> People<'a> {
        fn age_of(&self, empires: &'a str) -> &u32 {
            self.get_age.call(empires)
        }
    }

    let map =
        HashMap::from_iter([(String::from("john"), 42), (String::from("doe"), 33)].into_iter());
    let people = People {
        get_age: Capture(map).fun_ref(|m, p| m.get(p).unwrap_or(&0)),
    };

    assert_eq!(&42, people.age_of("john"));
    //assert_eq!(2, map.len()); // map is moved into the closure, this won't compile
    assert_eq!(&0, people.age_of("foo"));

    let map_back = people.get_age.into_captured_data();
    assert_eq!(2, map_back.len()); // map is moved out of the closure
}

#[test]
fn referencing_field() {
    struct People<'a> {
        get_age: ClosureRef<&'a HashMap<String, u32>, &'a str, u32>,
    }
    impl<'a> People<'a> {
        fn age_of(&self, empires: &'a str) -> &u32 {
            self.get_age.call(empires)
        }
    }

    let map =
        HashMap::from_iter([(String::from("john"), 42), (String::from("doe"), 33)].into_iter());
    let people = People {
        get_age: Capture(&map).fun_ref(|m, p| m.get(p).unwrap_or(&0)),
    };

    assert_eq!(&42, people.age_of("john"));
    assert_eq!(2, map.len()); // map is only referenced by the closure
    assert_eq!(&0, people.age_of("foo"));
}
