use std::time::Instant;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct City {
    position_x: i32,
    position_y: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TreeSpecifics {
    possible_destinations: [Vec<i32>; AMOUNT as usize],
    current_level: usize,
}

#[derive(Debug, Clone)]
pub struct Path {
    length: f64,
    route: Vec<i32>,
}

mod algorithms;
mod city_gen;

const AMOUNT: i32 = 10w;
const ITERATIONS: i32 = 100;

fn main() {
    let cities_distances: [[f64; AMOUNT as usize]; AMOUNT as usize] = city_gen::city_gen::cities_spawn();
    //println!("Spawning {} cities took {:?}", AMOUNT, now.elapsed());
    let cities_distances: [[f64; AMOUNT as usize]; AMOUNT as usize] = {
    let _now = Instant::now();

    let mut tot_naive_time: u128 = 0;
    let mut tot_rnn_time: u128 = 0;
    let mut tot_difference: f64 = 0.0;

    /*let cities_distances: [[f64; AMOUNT as usize]; AMOUNT as usize] = {
        [
            [
                0.0,
                5.385164807134504,
                4.58257569495584,
                3.605551275463989,
                4.47213595499958,
            ],
            [5.385164807134504, 0.0, 3.7416573867739413, 4.0, 3.0],
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
            ],
        ]
    };*/
    

    for _i in 0..ITERATIONS {
        let cities_distances: [[f64; AMOUNT as usize]; AMOUNT as usize] = city_gen::city_gen::cities_spawn();
        //println!("Spawning {} cities took {:?}", AMOUNT, now.elapsed());
        //println!("Distances included:  \n{:#?}", cities_distances);

    let now = Instant::now();
    let path_naive = algorithms::algorithms::naive(&cities_distances);
    println!("Solving Naïve solution took {:?}", now.elapsed());
    println!("Naïve solution: {:#?}", path_naive);
        let naive_time = now.elapsed().as_nanos();
        tot_naive_time = tot_naive_time + naive_time;
        //println!("Solving Naïve solution took {:?}", naive_time);
        //println!("Naïve solution: {:#?}", path_naive);

    let now = Instant::now();
    let path_repetitive = algorithms::algorithms::repetitive_nearest_neighbour(&cities_distances);
    println!("Solving RNN solution took {:?}", now.elapsed());
    println!("RNN solution: {:#?}", path_repetitive);
        let rnn_time = now.elapsed().as_nanos();
        tot_rnn_time = tot_rnn_time + rnn_time;
        //println!("Solving RNN solution took {:?}", rnn_time);
        //println!("RNN solution: {:#?}", path_rnn);

        let difference_naive_rnn = path_rnn.length - path_naive.length;
        tot_difference = tot_difference.clone() + difference_naive_rnn;
    }

    let avg_naive_time = tot_naive_time / AMOUNT as u128;
    let avg_rnn_time = tot_rnn_time / AMOUNT as u128;
    let avg_difference = tot_difference / AMOUNT as f64;

    println!("AVG Naïve time: {} \n AVG RNN time: {}", avg_naive_time, avg_rnn_time);
    println!("AVG rnn.length - naive.length: {}", avg_difference);

    //christofide_serdyukov(&cities_distances);
    };
}

/*fn christofide_serdyukov(cities_distances : &[[f64; 10];10]) {

}*/
