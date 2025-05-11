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

mod algorithms;
mod city_gen;

const AMOUNT: i32 = 5;

fn main() {
    let now = Instant::now();
    let cities_distances: [[f64; AMOUNT as usize]; AMOUNT as usize] = city_gen::city_gen::cities_spawn();
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
    let path_naive = algorithms::algorithms::naive(&cities_distances);
    println!("Solving Naïve solution took {:?}", now.elapsed());
    println!("Naïve solution: {:#?}", path_naive);

    let now = Instant::now();
    let path_repetitive = algorithms::algorithms::repetitive_nearest_neighbour(&cities_distances);
    println!("Solving RNN solution took {:?}", now.elapsed());
    println!("RNN solution: {:#?}", path_repetitive);

    //christofide_serdyukov(&cities_distances);
}

/*fn christofide_serdyukov(cities_distances : &[[f64; 10];10]) {

}*/