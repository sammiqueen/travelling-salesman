use rand::Rng;

use std::time::{Duration, Instant};

use ndarray::Array;
use ndarray::len_of;
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
struct Path {
    length: f64,
    route: [char; 10]
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

    let cities : [City; 10] = {
        let mut temp = [City {
            position_x: 0,
            position_y: 0
        }; 10];

        for i in 0..10 {

            let mut is_duplicate = false;

            for y in 0..10 {
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

    let cities_distances : Array::<f64, Ix2> = {
        let mut temp : Array::<f64, Ix2> = Array::<f64, Ix2>::zeros((10, 10));

        for i in len_of(temp, Axis(0)) {
            for y in len_of(temp, Axis(1)) {
                temp[[i, y]] = distance(cities[i], cities[y]);
            }
        }

        temp
    };

    cities_distances
}

fn distance(city_a : City, city_b : City) -> f64 {
    let x_dif : f64 = (city_a.position_x - city_b.position_x).abs().into();
    let y_dif : f64 = (city_a.position_y - city_b.position_y).abs().into();

    let distance : f64 = (x_dif + y_dif).sqrt();

    distance
}


fn naive(cities_distances : &Array::<f64, Ix2>) -> Path {
    println!("Hello from naive solution");



    return Path {
        length: 0.0,
        route: ['x'; 10]
    }
}

fn repetitive_nearest_neighbour(cities_distances : &Array::<f64, Ix2>) -> Path {
    println!("Hello from RNN solution");

    return Path {
        length: 0.0,
        route: ['x'; 10]
    }
}

/*fn christofide_serdyukov(cities_distances : &Array::<f64, Ix2>) {

}*/