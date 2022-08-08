use std::collections::HashMap;

#[test]
fn test_fn_ptr() {
    let mut router = OptimizedRouter::new();
    router.add_route("/hello", |r| {
        println!("{:?}", r);
        Response {
            code: 200,
            headers: Default::default(),
            body: vec![97, 97, 97],
        }
    });
    router.add_route("/bye", |r| {
        println!("{:?}", r);
        Response {
            code: 200,
            headers: Default::default(),
            body: vec![98, 98, 98],
        }
    });

    let req1 = Request {
        url: "/hello".to_string(),
        headers: Default::default(),
        method: "GET".to_string(),
        body: Default::default(),
    };
    let res1 = router.handle_request(&req1);
    assert_eq!(String::from_utf8(res1.body).unwrap(), "aaa");

    let req2 = Request {
        url: "/hola".to_string(),
        headers: Default::default(),
        method: "GET".to_string(),
        body: Default::default(),
    };
    let res1 = router.handle_request(&req2);
    assert_eq!(res1.code, 404);
}

type FnPtr = fn(&Request) -> Response;

struct OptimizedRouter {
    routes: HashMap<String, FnPtr>,
}

impl OptimizedRouter {
    fn new() -> Self {
        OptimizedRouter {
            routes: HashMap::new(),
        }
    }

    fn add_route(&mut self, url: &str, callback: FnPtr) {
        self.routes.insert(url.to_string(), callback);
    }

    fn handle_request(&self, r: &Request) -> Response {
        match self.routes.get(r.url.as_str()) {
            None => Response {
                code: 404,
                body: Default::default(),
                headers: Default::default(),
            },
            Some(cb) => cb(r),
        }
    }
}

#[test]
fn test_router_basic() {
    let mut router = BasicRouter::new();
    router.add_route("/hello", |r| {
        println!("{:?}", r);
        Response {
            code: 200,
            headers: Default::default(),
            body: vec![97, 97, 97],
        }
    });
    router.add_route("/bye", |r| {
        println!("{:?}", r);
        Response {
            code: 200,
            headers: Default::default(),
            body: vec![98, 98, 98],
        }
    });

    let req1 = Request {
        url: "/hello".to_string(),
        headers: Default::default(),
        method: "GET".to_string(),
        body: Default::default(),
    };
    let res1 = router.handle_request(&req1);
    assert_eq!(String::from_utf8(res1.body).unwrap(), "aaa");

    let req2 = Request {
        url: "/hola".to_string(),
        headers: Default::default(),
        method: "GET".to_string(),
        body: Default::default(),
    };
    let res1 = router.handle_request(&req2);
    assert_eq!(res1.code, 404);
}

type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

struct BasicRouter {
    routes: HashMap<String, BoxedCallback>,
}

impl BasicRouter {
    fn new() -> Self {
        BasicRouter {
            routes: HashMap::new(),
        }
    }

    fn add_route<C>(&mut self, url: &str, callback: C)
    where
        C: Fn(&Request) -> Response + 'static,
    {
        self.routes.insert(url.to_string(), Box::new(callback));
    }

    fn handle_request(&self, r: &Request) -> Response {
        match self.routes.get(r.url.as_str()) {
            None => Response {
                code: 404,
                body: Default::default(),
                headers: Default::default(),
            },
            Some(cb) => cb(r),
        }
    }
}

#[derive(Debug)]
struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

#[derive(Default)]
struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

#[test]
fn test_closure_copy_clone() {
    // Non-move closure with shared references to values which are Copy and Move is also Copy/Move
    let x = 1;
    let add = |y: i32| x + y;
    let add_copy = add; // add_copy is a copy of the original closure
    assert_eq!(5, add_copy(add(3)));

    // Non-move closure with exclusive reference is not Copy/Move since exclusive refs are also not
    let mut x = 0;
    let mut add = |y: i32| {
        x += y;
        x
    };
    let mut add_copy = add; // add_copy is now a result of "moving" add so add is no longer avail
                            // assert_eq!(3, add_copy(add(1))); // does NOT compile

    let mut greeting = "Hello ".to_string();
    let greet = move |name: &str| {
        greeting.push_str(name);
        greeting
    };
    let g1 = greet.clone()("Dumbledore");
    let g2 = greet("Logan");

    assert_eq!(g1, "Hello Dumbledore");
    assert_eq!(g2, "Hello Logan");
}

#[test]
fn test_sort() {
    let cities = vec![
        City {
            name: "Madrid".to_string(),
            population: 120_000,
            monster_risk: 0.5,
        },
        City {
            name: "London".to_string(),
            population: 220_000,
            monster_risk: 0.0,
        },
        City {
            name: "Mumbai".to_string(),
            population: 620_000,
            monster_risk: 0.0,
        },
        City {
            name: "JomocoLomo".to_string(),
            population: 20_000,
            monster_risk: 42.42,
        },
    ];
    assert_eq!(count_cities(&cities, has_monsters), 2);

    assert_eq!(count_cities(&cities, |city| city.name.starts_with('M')), 2);
}

#[test]
fn test_fn_mut() {
    let mut count = 0;
    let incr_func = || count += 1;
    call_twice(incr_func);
    assert_eq!(count, 2);
}

#[test]
fn test_drop() {
    let s = "Hi".to_owned();
    let dfunc = || drop(s);
    // call_twice(dfunc); -- NOT ALLOWED
}

fn call_twice<F>(mut func: F)
where
    F: FnMut(),
{
    func();
    func();
}

fn count_cities(cities: &Vec<City>, filter: fn(&City) -> bool) -> usize {
    let mut count = 0;
    for city in cities {
        if filter(city) {
            count += 1;
        }
    }
    count
}

fn has_monsters(city: &City) -> bool {
    city.monster_risk > 0.0
}

// fn start_sorting(mut cities: Vec<City>, mut stat: Statistic) -> JoinHandle<Vec<City>> {
//     let mut key_func = move |city: &City| {
//         stat.my();
//         city.get_statistic(stat)
//     };
//     thread::spawn(move || {
//         cities.sort_by_key(key_func);
//         cities
//     })
// }

enum Statistic {
    Population,
}

impl Statistic {
    fn my(&mut self) {}
}

struct City {
    name: String,
    population: i64,
    monster_risk: f64,
}

impl City {
    fn get_statistic(&self, stat: Statistic) -> i64 {
        match stat {
            Statistic::Population => self.population,
        }
    }
}
