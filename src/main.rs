use rand::Rng;

use std::time::Instant;

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
struct City {
    position_x: i32,
    position_y: i32,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
struct TreeSpecifics {
    possible_destinations: [Vec<i32>; AMOUNT as usize],
    current_level: usize,
}

#[derive(Debug)]
#[derive(Clone)]
struct Path {
    length: f64,
    route: Vec<i32>
}

const AMOUNT: i32 = 5;

fn main() {
    let now = Instant::now();
    //let cities_distances: [[f64; AMOUNT as usize]; AMOUNT as usize] = cities_spawn();
    //println!("Spawning {} cities took {:?}", AMOUNT, now.elapsed());
    let cities_distances: [[f64; AMOUNT as usize]; AMOUNT as usize] = {
        [
            [
                0.0,
                5.385164807134504,
                4.58257569495584,
                3.605551275463989,
                4.47213595499958,
            ],
            [
                5.385164807134504,
                0.0,
                3.7416573867739413,
                4.0,
                3.0,
            ],
            [
                4.58257569495584,
                3.7416573867739413,
                0.0,
                2.8284271247461903,
                3.605551275463989,
            ],
            [
                3.605551275463989,
                4.0,
                2.8284271247461903,
                0.0,
                3.3166247903554,
            ],
            [
                4.47213595499958,
                3.0,
                3.605551275463989,
                3.3166247903554,
                0.0,
            ]
        ]
    };
    println!("Distances included:  \n{:#?}", cities_distances);

    let now = Instant::now();
    let path_naive = naive(&cities_distances);
    println!("Solving Naïve solution took {:?}", now.elapsed());
    println!("Naïve solution: {:#?}", path_naive);

    let now = Instant::now();
    let path_repetitive = repetitive_nearest_neighbour(&cities_distances);
    println!("Solving RNN solution took {:?}", now.elapsed());
    println!("RNN solution: {:#?}", path_repetitive);

    //christofide_serdyukov(&cities_distances);
}

fn cities_spawn() -> [[f64; AMOUNT as usize]; AMOUNT as usize] {

    let mut rng = rand::rng();

    let cities: [City; AMOUNT as usize] = {
        let mut temp = [City {
            position_x: 0,
            position_y: 0
        }; AMOUNT as usize];

        for i in 0..temp.len() {

            let mut is_duplicate = false;

            for y in 0..temp.len() {
                match temp[i] {
                     val if val == temp[y] => is_duplicate = true,
                    City {
                        position_x: 0,
                        position_y: 0
                    } => is_duplicate = true,
                    _ => is_duplicate = false
                }
            }
            
            if is_duplicate {
                temp[i] = City {
                    position_x: rng.random_range(1..(AMOUNT * AMOUNT)),
                    position_y: rng.random_range(1..(AMOUNT * AMOUNT))
                };
            }
        }
        temp
    };

    let cities_distances: [[f64; AMOUNT as usize]; AMOUNT as usize] = {
        let mut temp: [[f64; AMOUNT as usize]; AMOUNT as usize] = [[0.0; AMOUNT as usize]; AMOUNT as  usize];

        for i in 0..cities.len() {
            for y in 0..cities.len() {
                temp[i][y] = distance(cities[i], cities[y]);
            }
        }

        temp
    };

    cities_distances
}

fn distance(city_a: City, city_b: City) -> f64 {
    let x_dif: f64 = (city_a.position_x - city_b.position_x).abs().into();
    let y_dif: f64 = (city_a.position_y - city_b.position_y).abs().into();

    let distance: f64 = (x_dif + y_dif).sqrt();

    distance
}

fn distance_between_two_cities (city_a: i32, city_b: i32, cities_distances: &[[f64; AMOUNT as usize]; AMOUNT as usize]) -> f64 {
    return cities_distances[city_a as usize][city_b as usize];
}

fn naive(cities_distances: &[[f64; AMOUNT as usize]; AMOUNT as usize]) -> Path {
    println!("Hello from naive solution");

    //list of all cities
    let cities: [i32; AMOUNT as usize] = {
        let mut temp = [0; AMOUNT as usize];
        for i in 0..AMOUNT {
            temp[i as usize] = i;
        }
        temp
    };

    //init cities vec, not including city 0 (starting city)
    let mut cities_vec = {
        let mut temp: Vec<i32> = vec!();
        for i in 1..(cities.len()) as usize {
            temp.push(cities[i])
        }
        temp
    };

    let starting_city = 0;

    //init path, empty path including none of the cities and having a length of 0
    let starting_path: Path = Path {
        length: 0.0,
        route: vec!(0),
    };

    let mut shortest_path = Path {
        length: f64::MAX,
        route: vec!()
    };

    //println!("{:?}", cities_vec);

    find_solution(&mut cities_vec, starting_city, starting_path.clone(), cities_distances, &mut shortest_path);

    fn find_solution (cities: &mut Vec<i32>, current_city: i32, mut path: Path, cities_distances: &[[f64; AMOUNT as usize]; AMOUNT as usize], shortest_path: &mut Path) {
        //println!("{:?} \n {:?}", shortest_path, path);
        println!("{:?}", cities);
        if cities.len() == 1 {
            //println!("{:?} \n {:?}", path.length, path.route);
            //when cities.len() == 1, add last city to the route and calculate new length. ALSO add 0th city (start position) to end of route and calculate new length
            path.length = path.length + distance_between_two_cities(path.route[path.route.len() - 1], cities[0], cities_distances);
            path.route.push(cities[0]);
            cities.pop();

            path.length = path.length + distance_between_two_cities(path.route[path.route.len() - 1], 0, cities_distances);
            path.route.push(0);
            println!("{:#?}", path);

            if path.length < shortest_path.length {
                *shortest_path = path.clone();
            }

            path.route.pop();
            path.length = path.length - distance_between_two_cities(path.route[path.route.len() - 1], 0, cities_distances);

            cities.push(path.route[path.route.len() - 1]);
            path.route.pop();
            path.length = path.length - distance_between_two_cities(path.route[path.route.len() - 1], cities[0], cities_distances);
            return
        }

        for i in 0..cities.len() {
            let city = cities[i];

            /*let mut new_path: Path = Path {
                length: path.length + distance_between_two_cities(current_city, city, cities_distances),
                route: path.route.clone()
            };
            new_path.route.push(city);*/

            let mut current_route = path.route.clone();
            current_route.push(city);
            let new_path: Path = Path {
                length: path.length + distance_between_two_cities(current_city, city, cities_distances),
                route: current_route
            };

            cities[i] = cities[cities.len() - 1];
            cities.pop();

            find_solution (cities, city, new_path, cities_distances, shortest_path);

            cities.push(city);
        }
    }
    shortest_path
}

fn repetitive_nearest_neighbour (cities_distances: &[[f64; AMOUNT as usize]; AMOUNT as usize]) -> Path {
    println!("Hello from RNN solution");

    let cities: [i32; AMOUNT as usize] = {
        let mut temp = [0; AMOUNT as usize];
        for i in 0..AMOUNT {
            temp[i as usize] = i;
        }
        temp
    };

    let mut shortest_path: Path = Path {
        length: f64::MAX,
        route: vec!()
    };

    for i in 0..AMOUNT {
        let starting_city = i;
        let mut cities_vec = {
            let mut temp: Vec<i32> = vec!();
            for y in 0..(cities.len()) as usize {
                if cities[y] != i {
                    temp.push(cities[y].clone())
                }
            }
            temp
        };
        //println!("{:#?} \n {}", cities_vec, starting_city);

        let mut chosen_path: Path = Path {
            length: 0.0,
            route: vec!(starting_city)
        };

        for _i in 0..AMOUNT - 1 {
            chosen_path.route.push({
                let mut shortest_distance = f64::MAX;
                let mut closest_known_city = -1;
                for y in 0..cities_vec.len() {
                    if distance_between_two_cities(chosen_path.route[chosen_path.route.len() - 1], cities_vec[y], cities_distances) < shortest_distance {
                        shortest_distance = distance_between_two_cities(chosen_path.route[chosen_path.route.len() - 1], cities_vec[y], cities_distances);
                        closest_known_city = cities_vec[y]
                    }
                }
                for i in 0..cities_vec.len() {
                    if cities_vec[i] == closest_known_city {
                        cities_vec[i] = cities_vec[cities_vec.len() - 1];
                        cities_vec.pop();
                        break
                    }
                }

            chosen_path.length = chosen_path.length + distance_between_two_cities(chosen_path.route[chosen_path.route.len() - 1], closest_known_city, cities_distances);
            closest_known_city
            });
            //println!("{:#?}, \n {}", cities_vec, chosen_path.route[chosen_path.route.len() - 1]);
        }
        chosen_path.length = chosen_path.length + distance_between_two_cities(chosen_path.route[chosen_path.route.len() - 1], starting_city, cities_distances);
        chosen_path.route.push(starting_city);
        //println!("{:#?}", chosen_path);

        if chosen_path.length < shortest_path.length {
            shortest_path = chosen_path.clone();
        }
    }
    shortest_path
}

/*fn christofide_serdyukov(cities_distances : &[[f64; 10];10]) {

}*/