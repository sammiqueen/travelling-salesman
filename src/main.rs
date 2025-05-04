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

const AMOUNT: i32 = 4;

fn main() {
    let now = Instant::now();
    let cities_distances: [[f64; AMOUNT as usize]; AMOUNT as usize] = cities_spawn();
    println!("Spawning {} cities took {:?}", AMOUNT, now.elapsed());
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

    fn find_solution (cities: &mut Vec<i32>, current_city: i32, path: Path, cities_distances: &[[f64; AMOUNT as usize]; AMOUNT as usize], shortest_path: &mut Path) {
        //println!("{:?} \n {:?}", shortest_path, path);
        if cities.len() == 0 {
            //println!("{:?} \n {:?}", path.length, path.route);
            if path.length < shortest_path.length {
                *shortest_path = path.clone();
            }
            return
        }

        for i in 0..cities.len() {
            let city = cities[i];

            let mut new_path: Path = Path {
                length: path.length + distance_between_two_cities(current_city, city, cities_distances),
                route: path.route.clone()
            };
            new_path.route.push(city);

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

    let mut cities_vec: Vec<i32> = vec!();
    for i in 0..cities.len() {
        cities_vec.push(cities[i]);
    }

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
                    temp.push(cities[y])
                }
            }
            temp
        };

        let mut init_path: Path = Path {
            length: f64::MAX,
            route: vec!()
        };

        //find_solution(&mut cities_vec, starting_city, init_path.clone(), cities_distances, &mut init_path);
        //println!("{:?}", init_path);
        if init_path.length < shortest_path.length {
            shortest_path = init_path.clone();
        }

        fn sammi_solution() {

        }

        /*fn find_solution (cities: &mut Vec<i32>, current_city: i32, path: Path, cities_distances: &[[f64; AMOUNT as usize]; AMOUNT as usize], found_path: &mut Path) {
            //println!("{:?} \n {:?}", shortest_path, path);
            if cities.len() == 0 {
                println!("{:?} \n {:?}", path.length, path.route);
                *found_path = path;
                return
            }

            println!("{:?}", cities);
    
            for i in 0..cities.len() {
                let next_city = {
                    let mut closest_city = 0;
                    let mut shortest_distance: f64 = f64::MAX;
                    for i in 0..cities.len() {
                        if shortest_distance > distance_between_two_cities(current_city, cities[i], cities_distances) {
                            shortest_distance = distance_between_two_cities(current_city, cities[i], cities_distances);
                            closest_city = cities[i];
                        }
                    }
                    closest_city
                };
    
                let mut new_path: Path = Path {
                    length: path.length + distance_between_two_cities(current_city, next_city, cities_distances),
                    route: path.route.clone()
                };
                new_path.route.push(next_city);
    
                cities[i] = cities[cities.len() - 1];
                cities.pop();
    
                find_solution (cities, next_city, new_path, cities_distances, found_path);
            }
        }*/
    }


    shortest_path
}

/*fn christofide_serdyukov(cities_distances : &[[f64; 10];10]) {

}*/