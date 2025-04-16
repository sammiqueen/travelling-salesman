use rand::Rng;

use std::time::Instant;

use ndarray::Array;
use ndarray::Ix2;

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
struct Path {
    length: f64,
    route: [i32; 10]
}

fn main() {
    let now = Instant::now();
    let cities_distances: Array::<f64, Ix2> = cities_spawn();
    println!("Spawning 10 cities took {:?}", now.elapsed());
println!("Distances included:  \n{}", cities_distances);

    let now = Instant::now();
    let path_naive = naive(&cities_distances);
    println!("Solving Naïve solution took {:?}", now.elapsed());
    println!("Naïve solution: {:?}", path_naive);

    let now = Instant::now();
    let path_repetitive = repetitive_nearest_neighbour(&cities_distances);
    println!("Solving RNN solution took {:?}", now.elapsed());
    println!("RNN solution: {:?}", path_repetitive)

    //christofide_serdyukov(&cities_distances);
}

fn cities_spawn() -> Array::<f64, Ix2> {

    let mut rng = rand::rng();

    let cities: [City; 10] = {
        let mut temp = [City {
            position_x: 0,
            position_y: 0
        }; 10];

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
                    position_x: rng.random_range(1..100),
                    position_y: rng.random_range(1..100)
                };
            }
        }
        temp
    };

    let cities_distances: Array::<f64, Ix2> = {
        let mut temp: Array::<f64, Ix2> = Array::<f64, Ix2>::zeros((10, 10));

        for i in 0..cities.len() {
            for y in 0..cities.len() {
                temp[[i, y]] = distance(cities[i], cities[y]);
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

fn length_of_route(route: [i32; 10], cities_distances: &Array::<f64, Ix2>) -> f64 {
    let path_length: f64 = {
        let mut temp: f64 = 0.0;
        for i in 0..route.len() - 1 {
            temp = temp + cities_distances[[route[i] as usize, route[i + 1] as usize]];
        }
        temp
    };

    path_length
}

fn naive(cities_distances: &Array::<f64, Ix2>) -> Path {
    println!("Hello from naive solution");

    let mut current_shortest_path: Path = Path {
        length: f64::MAX,
        route: [-1; 10]
    };

    let cities: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    let mut routes: [[i32;10];10] = [cities.clone(); 10];

    let mut current_path: Path;

    current_path.length = length_of_route(current_path.route, &cities_distances);

    if current_path.length < current_shortest_path.length {
        current_shortest_path = current_path.clone();
    }
    
    current_shortest_path
}

fn repetitive_nearest_neighbour (cities_distances: &Array::<f64, Ix2>) -> Path {
    println!("Hello from RNN solution");

    return Path {
        length: 0.0,
        route: [-1; 10]
    }
}

/*fn christofide_serdyukov(cities_distances : &Array::<f64, Ix2>) {

}*/